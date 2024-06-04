mod ice_cream;
mod ice_cream_shop;
mod order;

use actix::prelude::*;
use ice_cream_shop::{AddIceCream, AddOrder, IceCreamShop, RemoveOrder};
use order::Order;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};

#[actix_rt::main]
async fn main() {
    let ice_cream_shop = setup_ice_cream_shop().await;
    start_server(ice_cream_shop).await;
}

async fn setup_ice_cream_shop() -> Addr<IceCreamShop> {
    let ice_cream_shop = IceCreamShop::new().start();
    add_initial_ice_creams(&ice_cream_shop).await;
    ice_cream_shop
}

async fn add_initial_ice_creams(ice_cream_shop: &Addr<IceCreamShop>) {
    add_ice_cream(ice_cream_shop, "Vanilla", 10).await;
    add_ice_cream(ice_cream_shop, "Chocolate", 20).await;
}

async fn add_ice_cream(ice_cream_shop: &Addr<IceCreamShop>, flavor: &str, quantity: u32) {
    let res = ice_cream_shop
        .send(AddIceCream {
            flavor: flavor.to_string(),
            quantity,
        })
        .await;

    match res {
        Ok(_) => println!("{} ice cream added successfully", flavor),
        Err(_) => println!("Failed to add {} ice cream", flavor),
    }
}

async fn start_server(ice_cream_shop: Addr<IceCreamShop>) {
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("\nWaiting for connections!\n");
    let mut next_client_id = 1;

    loop {
        let (stream, _) = listener.accept().await.unwrap();
        let mut reader = BufReader::new(stream);
        let mut line = String::new();
        reader.read_line(&mut line).await.unwrap();
        let ice_cream_shop = ice_cream_shop.clone();
        match line.trim() {
            "CLIENTE" => {
                let client_id = next_client_id;
                next_client_id += 1;
                println!("[Client {}] Client connected", client_id);
                let (tx, rx) = tokio::sync::mpsc::channel(100);
                tokio::spawn(async move {
                    handle_client(&ice_cream_shop, &mut reader, client_id, tx, rx).await;
                });
            }
            "HELADERO" => {
                println!("[Ice cream maker] Ice cream maker connected");
                tokio::spawn(async move {
                    handle_ice_cream_maker(&ice_cream_shop, &mut reader).await;
                });
            }
            _ => {
                println!("[Unknown] Unknown client type");
            }
        }
    }
}

async fn handle_client(
    ice_cream_shop: &Addr<IceCreamShop>,
    reader: &mut BufReader<TcpStream>,
    client_id: u32,
    tx: tokio::sync::mpsc::Sender<String>,
    mut rx: tokio::sync::mpsc::Receiver<String>,
) {
    let mut line = String::new();
    while reader.read_line(&mut line).await.unwrap() > 0 {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            let flavor = parts[0].to_string();
            if let Ok(quantity) = parts[1].parse::<u32>() {
                let order = Order::new(flavor, quantity, tx.clone());

                println!(
                    "[Client {}] Received order: {} of {}",
                    client_id,
                    order.quantity(),
                    order.flavor()
                );

                let res = ice_cream_shop.send(AddOrder { order }).await;
                match res {
                    Ok(_) => {
                        println!("[Client {}] Order enqueue successfully \n", client_id);
                        let response = "Order processed successfully\n";
                        reader
                            .get_mut()
                            .write_all(response.as_bytes())
                            .await
                            .unwrap();
                    }
                    Err(_) => {
                        println!("[Client {}] Failed to process order \n", client_id);
                        let response = "Failed to process order\n";
                        reader
                            .get_mut()
                            .write_all(response.as_bytes())
                            .await
                            .unwrap();
                    }
                }
            }
        }
        line.clear();
    }

    while let Some(message) = rx.recv().await {
        println!(
            "[Client {}] Received message from channel: {}",
            client_id, message
        ); // Debug print

        let parts: Vec<&str> = message.split(',').collect();
        if parts.len() == 2 {
            let flavor = parts[0];
            let quantity = parts[1];

            println!(
                "[Client {}] Order completed: {} of {}",
                client_id, quantity, flavor
            );

            let response = format!("{},{}\n", quantity, flavor);
            reader
                .get_mut()
                .write_all(response.as_bytes())
                .await
                .unwrap();
        }
    }
}

async fn handle_ice_cream_maker(
    ice_cream_shop: &Addr<IceCreamShop>,
    reader: &mut BufReader<TcpStream>,
) {
    loop {
        // Wait for an order to become available
        let order = loop {
            if let Some(order) = ice_cream_shop.send(RemoveOrder).await.unwrap() {
                break order;
            }

            //TODO Remove Busy Wait later
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        };

        // Send the order quantity to the ice cream maker
        let order_quantity = format!("{}\n", order.quantity());
        reader
            .get_mut()
            .write_all(order_quantity.as_bytes())
            .await
            .expect("Failed to send order");

        // Wait for the response from the ice cream maker
        let mut response = String::new();
        reader
            .read_line(&mut response)
            .await
            .expect("Failed to read response");

        // Notify the client
        let message = format!("{},{}", order.flavor(), order.quantity());
        println!("Sending message to client: {}", message); // Debug print
        order.tx().send(message).await.unwrap();
    }
}
