use std::io::prelude::*;
use std::net::TcpStream;

fn main() {
    let stream = connect_to_server();
    let mut reader = std::io::BufReader::new(&stream);

    loop {
        // Request an order at the start of the loop
        request_order(&mut reader);

        println!("Waiting for order...");
        let quantity = get_order_quantity(&mut reader);

        println!("\nReceived order: {}", quantity);
        process_order(quantity);
        println!("Finished order: {}\n", quantity);

        // Send confirmation after processing the order
        send_confirmation(&mut reader);
    }
}

fn connect_to_server() -> TcpStream {
    let mut stream =
        TcpStream::connect("localhost:3000").expect("Could not connect to ice cream shop");
    stream
        .write_all(b"HELADERO\n")
        .expect("Failed to send worker type");
    println!("Connected to ice cream shop");
    stream
}

fn request_order(reader: &mut std::io::BufReader<&TcpStream>) {
    reader
        .get_mut()
        .write_all(b"REQUEST\n")
        .expect("Failed to request order");
}

fn get_order_quantity(reader: &mut std::io::BufReader<&TcpStream>) -> u32 {
    let mut quantity_str = String::new();
    reader
        .read_line(&mut quantity_str)
        .expect("Failed to read order quantity");
    quantity_str
        .trim()
        .parse()
        .expect("Failed to parse order quantity")
}

fn process_order(quantity: u32) {
    let sleep_duration = std::time::Duration::from_secs(quantity as u64);
    std::thread::sleep(sleep_duration);
}

fn send_confirmation(reader: &mut std::io::BufReader<&TcpStream>) {
    reader
        .get_mut()
        .write_all(b"CONFIRM\n")
        .expect("Failed to send confirmation");
}
