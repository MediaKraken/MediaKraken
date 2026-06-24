use telnet::{Event, Telnet};

pub async fn telnet_run_command(
    ip_address: String,
    port_number: u16,
    command_string: String,
    returned_data_blocks: u8,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut telnet =
        Telnet::connect((ip_address, port_number), 256).map_err(|e| format!("Couldn't connect to the server... {e}"))?;
    telnet
        .write(command_string.as_bytes())
        .map_err(|e| format!("Write error: {e}"))?;
    println!("after write");
    for _i  in 0..returned_data_blocks {
        let event = telnet.read().map_err(|e| format!("Read error: {e}"))?;
        if let Event::Data(buffer) = event {
            println!("Received: {}", String::from_utf8_lossy(&buffer));
        }
    }
    Ok(())
}
