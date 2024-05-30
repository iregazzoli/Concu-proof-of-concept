mod client;
use client::Client;
use serde_json::to_string;
use shared::order::Order;
use std::env;
use std::io::prelude::*;
use std::net::TcpStream;

fn main() {
    let orders_file_path = get_orders_file_path();
    let mut client = setup_client(&orders_file_path);
    let stream = connect_to_server();
    let mut reader = std::io::BufReader::new(&stream);

    let client_id = read_client_id(&mut reader);
    client.assign_id_to_orders(Some(client_id));

    process_orders(&mut client, &mut reader);
    println!("All orders have been successfully placed!");
}

fn get_orders_file_path() -> String {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <orders_file_name>", args[0]);
        std::process::exit(1);
    }
    let orders_file_name = &args[1];
    format!("orders/{}", orders_file_name)
}

fn setup_client(orders_file_path: &str) -> Client {
    let orders = Client::load_orders_from_file(orders_file_path);
    Client::new(orders)
}

fn connect_to_server() -> TcpStream {
    let mut stream =
        TcpStream::connect("localhost:3000").expect("Could not connect to ice cream shop");
    stream
        .write_all(b"CLIENTE\n")
        .expect("Failed to send client type");
    stream
}

fn read_client_id(reader: &mut std::io::BufReader<&TcpStream>) -> u32 {
    let mut id_message = String::new();
    reader
        .read_line(&mut id_message)
        .expect("Failed to read client ID");
    id_message
        .trim()
        .parse()
        .expect("Failed to parse client ID")
}

fn process_orders(client: &mut Client, reader: &mut std::io::BufReader<&TcpStream>) {
    while let Some(order) = client.place_order() {
        send_order(reader, &order);
        wait_for_response(reader);
    }
}

fn send_order(reader: &mut std::io::BufReader<&TcpStream>, order: &Order) {
    let order_json = to_string(&order).expect("Failed to serialize order");
    let order_json = format!("{}\n", order_json);
    reader
        .get_mut()
        .write_all(order_json.as_bytes())
        .expect("Failed to send order");
    println!("Order placed: {} of {}", order.quantity, order.flavor);
}

fn wait_for_response(reader: &mut std::io::BufReader<&TcpStream>) {
    let mut response = String::new();
    reader
        .read_line(&mut response)
        .expect("Failed to read response");
    println!("Received response: {}", response.trim());
}
