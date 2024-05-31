mod client;
use client::Client;
use std::env;
use std::io::prelude::*;
use std::net::TcpStream;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <orders_file_name>", args[0]);
        std::process::exit(1);
    }
    let orders_file_name = &args[1];
    let orders_file_path = format!("orders/{}", orders_file_name);

    let mut client = Client::new(Client::load_orders_from_file(&orders_file_path));
    let stream = connect_to_server();
    let mut reader = std::io::BufReader::new(&stream);

    process_orders(&mut client, &mut reader);
    wait_for_ice_cream_maker(&mut reader);
    println!("All orders have been successfully placed!");

    // Close the stream to disconnect
    drop(stream);
}

fn wait_for_ice_cream_maker(reader: &mut std::io::BufReader<&TcpStream>) {
    println!("\nWaiting to be served 🍦\n");
    let mut response = String::new();
    while response.trim() != "HELADERO CONNECTED" {
        reader
            .read_line(&mut response)
            .expect("Failed to read response");
    }
    println!("Ice cream maker connected");
}

fn process_orders(client: &mut Client, reader: &mut std::io::BufReader<&TcpStream>) {
    while let Some((flavor, quantity)) = client.place_order() {
        send_order(reader, &flavor, quantity);
        wait_for_response(reader);
    }
}

fn send_order(reader: &mut std::io::BufReader<&TcpStream>, flavor: &str, quantity: u32) {
    let order = format!("{} {}\n", flavor, quantity);
    reader
        .get_mut()
        .write_all(order.as_bytes())
        .expect("Failed to send order");
    println!("Order placed: {} of {}", quantity, flavor);
}

fn wait_for_response(reader: &mut std::io::BufReader<&TcpStream>) {
    let mut response = String::new();
    reader
        .read_line(&mut response)
        .expect("Failed to read response");
    println!("Received response: {}", response.trim());
}

fn connect_to_server() -> TcpStream {
    let server_address = "127.0.0.1:3000";
    let mut stream = TcpStream::connect(server_address).expect("Could not connect to server");

    // Send the client type to the server
    stream
        .write_all(b"CLIENTE\n")
        .expect("Failed to send client type to server");

    stream
}
