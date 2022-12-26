#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

pub async fn mk_lib_network_external_ip() {
    let response = reqwest::get("https://myexternalip.com/raw").await?;
    let content = response.bytes().await?;
    Ok(str::from_utf8(&content).unwrap().to_string())
}

/*
"https://icanhazip.com/",
"https://myexternalip.com/raw",
"https://ifconfig.io/ip",
"https://ipecho.net/plain",
"https://checkip.amazonaws.com/",
"https://ident.me/",
"http://whatismyip.akamai.com/",
"https://myip.dnsomatic.com/",
"https://diagnostic.opendns.com/myip",
 */
