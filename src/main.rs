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
    let orders = Client::load_orders_from_file(&orders_file_path);
    let mut client = Client::new(orders);

    // Conectar al servidor de la heladería
    let stream = TcpStream::connect("localhost:3000").expect("Could not connect to ice cream shop");

    println!("Successfully connected to the ice cream shop!");

    // Crear un BufReader para el stream
    let mut reader = std::io::BufReader::new(&stream);

    // Leer el ID del cliente del servidor
    let mut id_message = String::new();
    reader
        .read_line(&mut id_message)
        .expect("Failed to read client ID");
    let client_id: u32 = id_message
        .trim()
        .parse()
        .expect("Failed to parse client ID");

    // Asignar el ID del cliente a todas las órdenes
    client.assign_id_to_orders(Some(client_id));

    while let Some(order) = client.place_order() {
        // Serializar la orden a JSON
        let order_json = to_string(&order).expect("Failed to serialize order");

        // Agregar una nueva línea al final de cada orden
        let order_json = format!("{}\n", order_json);

        // Enviar la orden al servidor de la heladería
        reader
            .get_mut()
            .write_all(order_json.as_bytes())
            .expect("Failed to send order");

        println!("Order placed: {} of {}", order.quantity, order.flavor);

        // Esperar una respuesta del servidor
        let mut response = String::new();
        reader
            .read_line(&mut response)
            .expect("Failed to read response");
        println!("Received response: {}", response.trim());
    }

    println!("All orders have been successfully placed!");
}
