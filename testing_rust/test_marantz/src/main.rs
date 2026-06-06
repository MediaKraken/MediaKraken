use telnet::{Telnet, Event};

fn main() {
    let mut telnet = Telnet::connect(("192.168.1.209", 23), 256)
            .expect("Couldn't connect to the server...");
    println!("after connect");
    let telnet_buffer = "MVDOWN".as_bytes();
    telnet.write(telnet_buffer).expect("Read error");
    println!("after write");
    loop {
        let event = telnet.read_nonblocking().expect("Read error");

        if let Event::Data(buffer) = event {
            // Debug: print the data buffer
            println!("{:?}", buffer);
            println!("Received: {}", String::from_utf8_lossy(&buffer));
            // process the data buffer
        }

        // Do something else ...
    }
}