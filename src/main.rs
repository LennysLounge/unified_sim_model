use std::net::UdpSocket;

mod messages;

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:0").expect("Cannot bind udp socket");
    socket
        .connect("127.0.0.1:9000")
        .expect("Cannot connect socket");

    println!("Sending register request");
    socket
        .send(&messages::register_request("", 1000, ""))
        .expect("Cannot send");

    loop {
        println!("waiting for packet");
        let mut buf = [0u8; 2048];
        match socket.recv(&mut buf) {
            Ok(size) => {
                println!("{size} bytes read");
                match messages::read_response(&buf) {
                    Ok(response) => match response {
                        r => println!("{r:?}"),
                    },
                    Err(_) => println!("failed to parse the response"),
                };
            }
            Err(_) => {
                println!("failed to receive data");
                break;
            }
        }
    }
}
