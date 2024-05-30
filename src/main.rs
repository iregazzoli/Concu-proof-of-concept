mod ice_cream;
mod ice_cream_shop;

use actix::prelude::*;
use ice_cream_shop::{AddClient, AddIceCream, IceCreamShop};
use serde_json::from_slice;
use shared::order::Order;
use std::net::SocketAddr;
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
        let (mut stream, addr) = listener.accept().await.unwrap();
        println!("[{:?}] Client connected", addr);

        let client_id = next_client_id;
        next_client_id += 1;

        let id_message = format!("{}\n", client_id);
        stream.write_all(id_message.as_bytes()).await.unwrap();

        let mut reader = BufReader::new(stream);
        handle_client(&ice_cream_shop, &mut reader, addr, client_id).await;
    }
}

async fn handle_client(
    ice_cream_shop: &Addr<IceCreamShop>,
    _reader: &mut BufReader<TcpStream>,
    addr: SocketAddr,
    client_id: usize,
) {
    println!("[{:?}] Client {} connected", addr, client_id);

    // Enqueue the client
    let res = ice_cream_shop
        .send(AddClient {
            client_id: client_id.to_string(),
        })
        .await;
    match res {
        Ok(_) => {
            println!("[{:?}] Client {} enqueued successfully \n", addr, client_id);
        }
        Err(_) => {
            println!("[{:?}] Failed to enqueue client {} \n", addr, client_id);
        }
    }
}
