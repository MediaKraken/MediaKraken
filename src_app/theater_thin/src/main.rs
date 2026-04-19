use fltk::{app, enums::Color, prelude::*, window};
use serde_json::{Value, json};
use std::{
    collections::HashMap,
    env, fs,
    io::{ErrorKind, Read, Write},
    net::{TcpListener, TcpStream},
    os::unix::net::UnixStream,
    path::{Path, PathBuf},
    process::{Child, Command},
    sync::{Arc, Mutex},
    thread,
    time::{Duration, Instant},
};

const HTTP_BIND_ADDR: &str = "127.0.0.1:7878";
const ALLOWED_HOSTS: &[&str] = &[
    "127.0.0.1",
    "127.0.0.1:7878",
    "localhost",
    "localhost:7878",
];
const REQUEST_HEADER_LIMIT: usize = 8 * 1024;
const REQUEST_TIMEOUT: Duration = Duration::from_secs(5);
const MPV_READY_TIMEOUT: Duration = Duration::from_secs(10);
const MPV_SOCKET_WRITE_TIMEOUT: Duration = Duration::from_secs(2);
const MEDIA_ROOT_ENV: &str = "THEATER_THIN_MEDIA_ROOT";
const AUTH_TOKEN_ENV: &str = "THEATER_THIN_TOKEN";

struct MpvController {
    child: Mutex<Option<Child>>,
    socket_path: PathBuf,
}

impl MpvController {
    fn spawn(window_handle: u64, socket_path: PathBuf) -> std::io::Result<Self> {
        let _ = fs::remove_file(&socket_path);
        let child = Command::new("mpv")
            .arg(format!("--wid={window_handle}"))
            .arg("--idle=yes")
            .arg("--force-window=yes")
            .arg(format!("--input-ipc-server={}", socket_path.display()))
            .spawn()?;
        Ok(Self {
            child: Mutex::new(Some(child)),
            socket_path,
        })
    }

    fn wait_ready(&self) -> Result<(), String> {
        let deadline = Instant::now() + MPV_READY_TIMEOUT;
        loop {
            if self.socket_path.exists() && UnixStream::connect(&self.socket_path).is_ok() {
                return Ok(());
            }
            if Instant::now() >= deadline {
                return Err("mpv IPC socket did not become ready".to_string());
            }
            thread::sleep(Duration::from_millis(50));
        }
    }

    fn send(&self, command: &str) -> Result<(), String> {
        if matches!(self.child.lock(), Ok(guard) if guard.is_none()) {
            return Err("mpv is not running".to_string());
        }
        let mut stream = UnixStream::connect(&self.socket_path)
            .map_err(|e| format!("connect: {e}"))?;
        let _ = stream.set_write_timeout(Some(MPV_SOCKET_WRITE_TIMEOUT));
        stream
            .write_all(command.as_bytes())
            .map_err(|e| format!("write: {e}"))?;
        Ok(())
    }

    fn shutdown(&self) {
        if let Ok(mut guard) = self.child.lock() {
            if let Some(mut child) = guard.take() {
                let _ = child.kill();
                let _ = child.wait();
            }
        }
        let _ = fs::remove_file(&self.socket_path);
    }
}

impl Drop for MpvController {
    fn drop(&mut self) {
        self.shutdown();
    }
}

fn runtime_socket_path() -> PathBuf {
    let dir = env::var_os("XDG_RUNTIME_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(env::temp_dir);
    dir.join(format!("theater_thin_mpv_{}.sock", std::process::id()))
}

fn media_root() -> Option<PathBuf> {
    env::var_os(MEDIA_ROOT_ENV)
        .map(PathBuf::from)
        .and_then(|p| p.canonicalize().ok())
}

fn auth_token() -> Option<String> {
    env::var(AUTH_TOKEN_ENV).ok().filter(|s| !s.is_empty())
}

fn main() {
    // mpv --wid only works with X11 windows; warn early on Wayland-only sessions.
    if env::var_os("WAYLAND_DISPLAY").is_some() && env::var_os("DISPLAY").is_none() {
        eprintln!("warning: running under Wayland without XWayland; mpv embedding will not work");
    }

    let _ = mk_lib_network::mk_lib_network_mediakraken::mk_lib_network_find_mediakraken_server();

    let app = app::App::default().with_scheme(app::AppScheme::Gtk);
    let mut win = window::Window::new(100, 100, 800, 600, "Media Player");
    let mut mpv_win = window::Window::new(10, 10, 780, 520, "");
    mpv_win.end();
    mpv_win.set_color(Color::Black);
    win.end();
    win.show();
    win.make_resizable(true);

    let socket_path = runtime_socket_path();
    let handle = mpv_win.raw_handle();
    let controller = match MpvController::spawn(handle as u64, socket_path) {
        Ok(c) => Arc::new(c),
        Err(e) => {
            eprintln!("Failed to launch mpv: {e}");
            return;
        }
    };

    if let Err(e) = controller.wait_ready() {
        eprintln!("mpv readiness check failed: {e}");
    }

    let token = auth_token();
    let media_root = media_root();
    if media_root.is_none() {
        eprintln!("warning: {MEDIA_ROOT_ENV} not set; /api/play will be rejected");
    }
    if token.is_none() {
        eprintln!("warning: {AUTH_TOKEN_ENV} not set; control endpoints will be rejected");
    }

    run_http_api(Arc::clone(&controller), token, media_root);

    if let Err(error) = app.run() {
        eprintln!("Application exited with error: {error}");
    }

    controller.shutdown();
}

fn run_http_api(
    controller: Arc<MpvController>,
    token: Option<String>,
    media_root: Option<PathBuf>,
) {
    thread::spawn(move || {
        let listener = match TcpListener::bind(HTTP_BIND_ADDR) {
            Ok(l) => l,
            Err(e) => {
                eprintln!("Failed to bind HTTP API at {HTTP_BIND_ADDR}: {e}");
                return;
            }
        };

        for stream_result in listener.incoming() {
            match stream_result {
                Ok(stream) => {
                    let controller = Arc::clone(&controller);
                    let token = token.clone();
                    let media_root = media_root.clone();
                    thread::spawn(move || {
                        if let Err(e) = handle_client(
                            stream,
                            &controller,
                            token.as_deref(),
                            media_root.as_deref(),
                        ) {
                            eprintln!("HTTP API request failed: {e}");
                        }
                    });
                }
                Err(e) => eprintln!("Failed to accept HTTP client: {e}"),
            }
        }
    });
}

fn handle_client(
    mut stream: TcpStream,
    controller: &MpvController,
    token: Option<&str>,
    media_root: Option<&Path>,
) -> std::io::Result<()> {
    stream.set_read_timeout(Some(REQUEST_TIMEOUT))?;
    stream.set_write_timeout(Some(REQUEST_TIMEOUT))?;

    let raw = read_request_head(&mut stream)?;
    if raw.is_empty() {
        return Ok(());
    }
    let head = String::from_utf8_lossy(&raw);
    let mut lines = head.lines();
    let Some(request_line) = lines.next() else {
        return Ok(());
    };

    let mut parts = request_line.split_whitespace();
    let method = parts.next().unwrap_or_default();
    let target = parts.next().unwrap_or_default();

    let mut host_ok = false;
    let mut auth_header: Option<String> = None;
    for line in lines {
        if line.is_empty() {
            break;
        }
        let Some((name, value)) = line.split_once(':') else {
            continue;
        };
        let value = value.trim();
        match name.trim().to_ascii_lowercase().as_str() {
            "host" => {
                if ALLOWED_HOSTS.iter().any(|h| h.eq_ignore_ascii_case(value)) {
                    host_ok = true;
                }
            }
            "authorization" => auth_header = Some(value.to_string()),
            _ => {}
        }
    }

    if !host_ok {
        return write_response(
            &mut stream,
            "400 Bad Request",
            &json!({"error": "invalid or missing Host header"}).to_string(),
        );
    }

    let (path, query) = split_target(target);
    let query_map = parse_query(query);

    let (status, body) = route_request(
        method,
        path,
        &query_map,
        controller,
        token,
        auth_header.as_deref(),
        media_root,
    );
    write_response(&mut stream, status, &body)
}

fn read_request_head(stream: &mut TcpStream) -> std::io::Result<Vec<u8>> {
    let mut buf = Vec::with_capacity(1024);
    let mut chunk = [0u8; 1024];
    loop {
        if buf.len() > REQUEST_HEADER_LIMIT {
            return Err(std::io::Error::new(
                ErrorKind::InvalidData,
                "request headers exceed limit",
            ));
        }
        let n = stream.read(&mut chunk)?;
        if n == 0 {
            break;
        }
        buf.extend_from_slice(&chunk[..n]);
        if buf.windows(4).any(|w| w == b"\r\n\r\n") {
            break;
        }
    }
    Ok(buf)
}

fn write_response(stream: &mut TcpStream, status: &str, body: &str) -> std::io::Result<()> {
    let resp = format!(
        "HTTP/1.1 {status}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
        body.len()
    );
    stream.write_all(resp.as_bytes())?;
    stream.flush()
}

fn route_request(
    method: &str,
    path: &str,
    query: &HashMap<String, String>,
    controller: &MpvController,
    token: Option<&str>,
    auth_header: Option<&str>,
    media_root: Option<&Path>,
) -> (&'static str, String) {
    if method == "GET" && path == "/api/health" {
        return (
            "200 OK",
            json!({"status": "ok", "service": "theater_thin"}).to_string(),
        );
    }

    if !is_authorized(token, auth_header) {
        return (
            "401 Unauthorized",
            json!({"error": "unauthorized"}).to_string(),
        );
    }

    match (method, path) {
        ("POST", "/api/play") => {
            let Some(path_param) = query.get("path") else {
                return (
                    "400 Bad Request",
                    json!({"error": "missing required query parameter: path"}).to_string(),
                );
            };
            let Some(root) = media_root else {
                return (
                    "503 Service Unavailable",
                    json!({"error": "media root not configured"}).to_string(),
                );
            };
            let resolved = match resolve_within(root, path_param) {
                Ok(p) => p,
                Err(e) => {
                    return (
                        "400 Bad Request",
                        json!({"error": "invalid path", "detail": e}).to_string(),
                    );
                }
            };
            let cmd = json!({
                "command": ["loadfile", resolved.to_string_lossy(), "replace"],
            })
            .to_string()
                + "\n";
            send_with_status(
                controller,
                &cmd,
                "playing",
                json!({"path": resolved.to_string_lossy()}),
            )
        }
        ("POST", "/api/pause") => send_with_status(
            controller,
            &json!({"command": ["set_property", "pause", true]}).to_string(),
            "paused",
            Value::Null,
        ),
        ("POST", "/api/resume") => send_with_status(
            controller,
            &json!({"command": ["set_property", "pause", false]}).to_string(),
            "resumed",
            Value::Null,
        ),
        ("POST", "/api/stop") => send_with_status(
            controller,
            &json!({"command": ["stop"]}).to_string(),
            "stopped",
            Value::Null,
        ),
        _ => (
            "404 Not Found",
            json!({
                "error": "not found",
                "supported": [
                    "GET /api/health",
                    "POST /api/play?path=<file>",
                    "POST /api/pause",
                    "POST /api/resume",
                    "POST /api/stop",
                ],
            })
            .to_string(),
        ),
    }
}

fn send_with_status(
    controller: &MpvController,
    command: &str,
    ok_status: &str,
    extra: Value,
) -> (&'static str, String) {
    let payload = if !command.ends_with('\n') {
        format!("{command}\n")
    } else {
        command.to_string()
    };
    match controller.send(&payload) {
        Ok(()) => {
            let mut value = json!({"status": ok_status});
            if let (Some(obj), Some(extra_obj)) = (value.as_object_mut(), extra.as_object()) {
                for (k, v) in extra_obj {
                    obj.insert(k.clone(), v.clone());
                }
            }
            ("200 OK", value.to_string())
        }
        Err(e) => (
            "500 Internal Server Error",
            json!({"error": "mpv command failed", "detail": e}).to_string(),
        ),
    }
}

fn is_authorized(expected: Option<&str>, header: Option<&str>) -> bool {
    let Some(expected) = expected else {
        return false;
    };
    let Some(header) = header else {
        return false;
    };
    let provided = header.strip_prefix("Bearer ").unwrap_or(header).trim();
    constant_time_eq(expected.as_bytes(), provided.as_bytes())
}

fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut diff = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        diff |= x ^ y;
    }
    diff == 0
}

fn resolve_within(root: &Path, requested: &str) -> Result<PathBuf, String> {
    let candidate = Path::new(requested);
    let joined = if candidate.is_absolute() {
        candidate.to_path_buf()
    } else {
        root.join(candidate)
    };
    let canonical = joined
        .canonicalize()
        .map_err(|e| format!("canonicalize: {e}"))?;
    if !canonical.starts_with(root) {
        return Err("path escapes media root".to_string());
    }
    if !canonical.is_file() {
        return Err("not a regular file".to_string());
    }
    Ok(canonical)
}

fn split_target(target: &str) -> (&str, &str) {
    target.split_once('?').unwrap_or((target, ""))
}

fn parse_query(query: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for pair in query.split('&') {
        if pair.is_empty() {
            continue;
        }
        let (key, value) = pair.split_once('=').unwrap_or((pair, ""));
        map.insert(url_decode(key), url_decode(value));
    }
    map
}

fn url_decode(value: &str) -> String {
    let bytes = value.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'+' => {
                out.push(b' ');
                i += 1;
            }
            b'%' if i + 2 < bytes.len() => {
                let hex = std::str::from_utf8(&bytes[i + 1..i + 3]).unwrap_or("");
                match u8::from_str_radix(hex, 16) {
                    Ok(byte) => {
                        out.push(byte);
                        i += 3;
                    }
                    Err(_) => {
                        out.push(b'%');
                        i += 1;
                    }
                }
            }
            other => {
                out.push(other);
                i += 1;
            }
        }
    }
    String::from_utf8_lossy(&out).into_owned()
}
