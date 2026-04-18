use fltk::{app, enums::Color, prelude::*, window};
use mk_lib_network;
use std::{
    collections::HashMap,
    fs,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    os::unix::net::UnixStream,
    path::Path,
    process::{Child, Command},
    sync::{Arc, Mutex},
    thread,
};

const HTTP_BIND_ADDR: &str = "127.0.0.1:7878";
const MPV_IPC_PATH: &str = "/tmp/theater_thin_mpv.sock";

fn main() {
    let _server_list =
        mk_lib_network::mk_lib_network_mediakraken::mk_lib_network_find_mediakraken_server();

    let app = app::App::default().with_scheme(app::AppScheme::Gtk);
    let mut win = window::Window::new(100, 100, 800, 600, "Media Player");

    // Inner window used as embedded media player target.
    let mut mpv_win = window::Window::new(10, 10, 780, 520, "");
    mpv_win.end();
    mpv_win.set_color(Color::Black);

    win.end();
    win.show();
    win.make_resizable(true);

    let handle = mpv_win.raw_handle();
    let mpv_child = start_mpv(handle as u64);
    let mpv_child_shared = Arc::new(Mutex::new(mpv_child));

    run_http_api(Arc::clone(&mpv_child_shared));

    if let Err(error) = app.run() {
        eprintln!("Application exited with error: {error}");
    }
}

fn start_mpv(window_handle: u64) -> Option<Child> {
    let _ = fs::remove_file(MPV_IPC_PATH);

    let mut command = Command::new("mpv");
    command
        .arg(format!("--wid={window_handle}"))
        .arg("--idle=yes")
        .arg("--force-window=yes")
        .arg(format!("--input-ipc-server={MPV_IPC_PATH}"));

    match command.spawn() {
        Ok(child) => Some(child),
        Err(error) => {
            eprintln!("Failed to launch mpv: {error}");
            None
        }
    }
}

fn run_http_api(mpv_child: Arc<Mutex<Option<Child>>>) {
    thread::spawn(move || {
        let listener = match TcpListener::bind(HTTP_BIND_ADDR) {
            Ok(listener) => listener,
            Err(error) => {
                eprintln!("Failed to bind HTTP API at {HTTP_BIND_ADDR}: {error}");
                return;
            }
        };

        for stream_result in listener.incoming() {
            match stream_result {
                Ok(stream) => {
                    if let Err(error) = handle_client(stream, &mpv_child) {
                        eprintln!("HTTP API request failed: {error}");
                    }
                }
                Err(error) => {
                    eprintln!("Failed to accept HTTP client: {error}");
                }
            }
        }
    });
}

fn handle_client(
    mut stream: TcpStream,
    mpv_child: &Arc<Mutex<Option<Child>>>,
) -> Result<(), std::io::Error> {
    let mut buffer = [0_u8; 4096];
    let bytes_read = stream.read(&mut buffer)?;
    if bytes_read == 0 {
        return Ok(());
    }

    let request = String::from_utf8_lossy(&buffer[..bytes_read]);
    let mut lines = request.lines();
    let Some(request_line) = lines.next() else {
        return Ok(());
    };

    let mut request_parts = request_line.split_whitespace();
    let method = request_parts.next().unwrap_or_default();
    let target = request_parts.next().unwrap_or_default();

    let (path, query) = split_target(target);
    let query_map = parse_query(query);

    let (status_line, body) = route_request(method, path, &query_map, mpv_child);

    let response = format!(
        "{status_line}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
        body.len()
    );
    stream.write_all(response.as_bytes())?;
    stream.flush()?;

    Ok(())
}

fn route_request(
    method: &str,
    path: &str,
    query: &HashMap<String, String>,
    mpv_child: &Arc<Mutex<Option<Child>>>,
) -> (&'static str, String) {
    match (method, path) {
        ("GET", "/api/health") => (
            "HTTP/1.1 200 OK",
            "{\"status\":\"ok\",\"service\":\"theater_thin\"}".to_string(),
        ),
        ("POST", "/api/play") => {
            let Some(path_param) = query.get("path") else {
                return (
                    "HTTP/1.1 400 Bad Request",
                    "{\"error\":\"missing required query parameter: path\"}".to_string(),
                );
            };

            if !Path::new(path_param).exists() {
                return (
                    "HTTP/1.1 400 Bad Request",
                    format!(
                        "{{\"error\":\"file not found\",\"path\":{}}}",
                        to_json_string(path_param)
                    ),
                );
            }

            let command = format!(
                "{{\"command\":[\"loadfile\",{} ,\"replace\"]}}\n",
                to_json_string(path_param)
            );

            match send_mpv_ipc_command(&command, mpv_child) {
                Ok(()) => (
                    "HTTP/1.1 200 OK",
                    format!("{{\"status\":\"playing\",\"path\":{}}}", to_json_string(path_param)),
                ),
                Err(error) => (
                    "HTTP/1.1 500 Internal Server Error",
                    format!(
                        "{{\"error\":\"failed to send play command\",\"detail\":{}}}",
                        to_json_string(&error)
                    ),
                ),
            }
        }
        ("POST", "/api/pause") => {
            let command = "{\"command\":[\"set_property\",\"pause\",true]}\n";
            match send_mpv_ipc_command(command, mpv_child) {
                Ok(()) => ("HTTP/1.1 200 OK", "{\"status\":\"paused\"}".to_string()),
                Err(error) => (
                    "HTTP/1.1 500 Internal Server Error",
                    format!(
                        "{{\"error\":\"failed to pause\",\"detail\":{}}}",
                        to_json_string(&error)
                    ),
                ),
            }
        }
        ("POST", "/api/resume") => {
            let command = "{\"command\":[\"set_property\",\"pause\",false]}\n";
            match send_mpv_ipc_command(command, mpv_child) {
                Ok(()) => ("HTTP/1.1 200 OK", "{\"status\":\"resumed\"}".to_string()),
                Err(error) => (
                    "HTTP/1.1 500 Internal Server Error",
                    format!(
                        "{{\"error\":\"failed to resume\",\"detail\":{}}}",
                        to_json_string(&error)
                    ),
                ),
            }
        }
        ("POST", "/api/stop") => {
            let command = "{\"command\":[\"stop\"]}\n";
            match send_mpv_ipc_command(command, mpv_child) {
                Ok(()) => ("HTTP/1.1 200 OK", "{\"status\":\"stopped\"}".to_string()),
                Err(error) => (
                    "HTTP/1.1 500 Internal Server Error",
                    format!(
                        "{{\"error\":\"failed to stop\",\"detail\":{}}}",
                        to_json_string(&error)
                    ),
                ),
            }
        }
        _ => (
            "HTTP/1.1 404 Not Found",
            "{\"error\":\"not found\",\"supported\":[\"GET /api/health\",\"POST /api/play?path=<file>\",\"POST /api/pause\",\"POST /api/resume\",\"POST /api/stop\"]}".to_string(),
        ),
    }
}

fn send_mpv_ipc_command(
    command: &str,
    mpv_child: &Arc<Mutex<Option<Child>>>,
) -> Result<(), String> {
    {
        let lock = mpv_child
            .lock()
            .map_err(|_| "mpv process state lock poisoned".to_string())?;
        if lock.is_none() {
            return Err("mpv is not running".to_string());
        }
    }

    let mut stream = UnixStream::connect(MPV_IPC_PATH)
        .map_err(|error| format!("failed to connect to mpv socket: {error}"))?;
    stream
        .write_all(command.as_bytes())
        .map_err(|error| format!("failed to write to mpv socket: {error}"))?;
    Ok(())
}

fn split_target(target: &str) -> (&str, &str) {
    if let Some((path, query)) = target.split_once('?') {
        (path, query)
    } else {
        (target, "")
    }
}

fn parse_query(query: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for pair in query.split('&') {
        if pair.is_empty() {
            continue;
        }

        let (key, value) = if let Some((key, value)) = pair.split_once('=') {
            (key, value)
        } else {
            (pair, "")
        };

        map.insert(url_decode(key), url_decode(value));
    }
    map
}

fn url_decode(value: &str) -> String {
    let mut output = String::new();
    let mut chars = value.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '%' {
            let hi = chars.next();
            let lo = chars.next();
            match (hi, lo) {
                (Some(hi), Some(lo)) => {
                    let hex = [hi, lo].iter().collect::<String>();
                    match u8::from_str_radix(&hex, 16) {
                        Ok(byte) => output.push(char::from(byte)),
                        Err(_) => {
                            output.push('%');
                            output.push(hi);
                            output.push(lo);
                        }
                    }
                }
                _ => output.push('%'),
            }
        } else if ch == '+' {
            output.push(' ');
        } else {
            output.push(ch);
        }
    }

    output
}

fn to_json_string(input: &str) -> String {
    let escaped = input
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t");
    format!("\"{escaped}\"")
}
