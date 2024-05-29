mod client;
use client::Client;
use serde_json::to_string;
use std::env;
use std::io::prelude::*;
use std::net::TcpStream;

// correlo con cargo run order1.json
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <orders_file_name>", args[0]);
        std::process::exit(1);
    }
    let orders_file_name = &args[1];
    let orders_file_path = format!("orders/{}", orders_file_name);
    let mut orders = Client::load_orders_from_file(&orders_file_path);
    orders.reverse(); // Reverse the orders to use pop for getting the first order
    let mut client = Client::new(orders);

    // Conectar al servidor de la heladería
    let mut stream =
        TcpStream::connect("localhost:3000").expect("Could not connect to ice cream shop");

    println!("Successfully connected to the ice cream shop!");

    while let Some(order) = client.place_order() {
        // Serializar la orden a JSON
        let order_json = to_string(&order).expect("Failed to serialize order");

        // Agregar una nueva línea al final de cada orden
        let order_json = format!("{}\n", order_json);

        // Enviar la orden al servidor de la heladería
        stream
            .write_all(order_json.as_bytes())
            .expect("Failed to send order");

        println!("Order placed: {} of {}", order.quantity, order.flavor);
    }

    println!("All orders have been successfully placed!");
}
