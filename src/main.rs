use crate::acc_udp::AccAdapter;

mod acc_udp;
mod messages;
mod model;

fn main() {
    println!("Connecting to game");
    let acc_adapter = AccAdapter::new().expect("Cannot connect to game");

    if let Err(e) = acc_adapter
        .join_handle
        .join()
        .expect("Couldnt join connection thread")
    {
        println!("Connection failed because: {}", e);
    }
    println!("Connection done");
}
