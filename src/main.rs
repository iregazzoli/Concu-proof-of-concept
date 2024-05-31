mod ice_cream;
mod ice_cream_shop;

use actix::prelude::*;
use ice_cream_shop::{AddIceCream, AddOrder, IceCreamShop, RemoveOrder};
use serde_yaml;
use shared::order::Order;
use std::net::SocketAddr;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream as TokioTcpStream};

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
        let (stream, addr) = listener.accept().await.unwrap();
        let mut reader = BufReader::new(stream);
        let mut line = String::new();
        reader.read_line(&mut line).await.unwrap();
        let ice_cream_shop = ice_cream_shop.clone();
        match line.trim() {
            "CLIENTE" => {
                println!("[{:?}] Client connected", addr);
                let client_id = next_client_id;
                next_client_id += 1;
                tokio::spawn(async move {
                    handle_client(&ice_cream_shop, &mut reader, addr, client_id).await;
                });
            }
            "HELADERO" => {
                println!("[{:?}] Ice cream maker connected", addr);
                tokio::spawn(async move {
                    handle_ice_cream_maker(&ice_cream_shop, &mut reader, addr).await;
                });
            }
            _ => {
                println!("[{:?}] Unknown client type", addr);
            }
        }
    }
}

async fn handle_client(
    ice_cream_shop: &Addr<IceCreamShop>,
    reader: &mut BufReader<TokioTcpStream>,
    addr: SocketAddr,
    client_id: u32,
) {
    let mut line = String::new();
    while reader.read_line(&mut line).await.unwrap() > 0 {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            let flavor = parts[0].to_string();
            if let Ok(quantity) = parts[1].parse::<u32>() {
                let order = Order {
                    id: client_id,
                    flavor,
                    quantity,
                };

                println!(
                    "[{:?}] Received order: {} of {} from client {}",
                    addr, order.quantity, order.flavor, client_id
                );

                let res = ice_cream_shop.send(AddOrder { order }).await;
                match res {
                    Ok(_) => {
                        println!(
                            "[{:?}] Order from client {} enqueue successfully \n",
                            addr, client_id
                        );
                        let response = "Order processed successfully\n";
                        reader
                            .get_mut()
                            .write_all(response.as_bytes())
                            .await
                            .unwrap();
                    }
                    Err(_) => {
                        println!(
                            "[{:?}] Failed to process order from client {} \n",
                            addr, client_id
                        );
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
}

async fn handle_ice_cream_maker(
    ice_cream_shop: &Addr<IceCreamShop>,
    reader: &mut BufReader<TokioTcpStream>,
    addr: SocketAddr,
) {
    loop {
        // Wait for an order to become available
        let order = loop {
            if let Some(order) = ice_cream_shop.send(RemoveOrder).await.unwrap() {
                break order;
            }

            // If there are no orders, sleep for a short duration before checking again
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        };

        // Send the order to the ice cream maker
        let order_yaml = serde_yaml::to_string(&order).expect("Failed to serialize order");
        let order_yaml = format!("{}\n", order_yaml);
        reader
            .get_mut()
            .write_all(order_yaml.as_bytes())
            .await
            .expect("Failed to send order");

        // Wait for the response from the ice cream maker
        let mut response = String::new();
        reader
            .read_line(&mut response)
            .await
            .expect("Failed to read response");
    }
}
