use serde_json::from_slice;
use shared::order::Order;
use std::io::prelude::*;
use std::net::TcpStream;

fn main() {
    let stream = connect_to_server();
    let mut reader = std::io::BufReader::new(&stream);

    // Request an order as soon as the worker connects
    request_order(&mut reader);

    loop {
        let order = get_order(&mut reader);
        process_order(&order);
        send_confirmation(&mut reader);
        request_order(&mut reader);
    }
}

fn connect_to_server() -> TcpStream {
    let mut stream =
        TcpStream::connect("localhost:3000").expect("Could not connect to ice cream shop");
    stream
        .write_all(b"HELADERO\n")
        .expect("Failed to send worker type");
    stream
}

fn request_order(reader: &mut std::io::BufReader<&TcpStream>) {
    reader
        .get_mut()
        .write_all(b"REQUEST\n")
        .expect("Failed to request order");
}

fn get_order(reader: &mut std::io::BufReader<&TcpStream>) -> Order {
    let mut order_json = String::new();
    reader
        .read_line(&mut order_json)
        .expect("Failed to read order");
    from_slice(order_json.trim().as_bytes()).expect("Failed to parse order")
}

fn process_order(order: &Order) {
    let sleep_duration = std::time::Duration::from_secs(order.quantity as u64);
    std::thread::sleep(sleep_duration);
}

fn send_confirmation(reader: &mut std::io::BufReader<&TcpStream>) {
    reader
        .get_mut()
        .write_all(b"CONFIRM\n")
        .expect("Failed to send confirmation");
}
