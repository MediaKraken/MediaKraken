use telnet::{Event, Telnet};

pub async fn telnet_run_command(
    ip_address: String,
    port_number: u16,
    command_string: String,
    returned_data_blocks: u8,
) {
    let mut telnet =
        Telnet::connect((ip_address, port_number), 256).expect("Couldn't connect to the server...");
    telnet
        .write(command_string.as_bytes())
        .expect("Read error");
    println!("after write");
    for _i  in 0..returned_data_blocks {
        let event = telnet.read().expect("Read error");
        if let Event::Data(buffer) = event {
            println!("Received: {}", String::from_utf8_lossy(&buffer));
        }
    }
}
