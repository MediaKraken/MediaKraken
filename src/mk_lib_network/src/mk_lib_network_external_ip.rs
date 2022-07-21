#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

// https://github.com/mellon85/external-ip
// external-ip = "4.1.0"

pub async fn mk_lib_network_external_ip() {
    //     let result = external_ip::get_ip();
    //     let value : Option<IpAddr> = block_on(result);
    let response = reqwest::get(url).await?;
    println!("response: {:?}", response);
    let content = response.bytes().await?;
    Ok(str::from_utf8(&content).unwrap().to_string())
    //     value
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
