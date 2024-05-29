mod ice_cream;
mod ice_cream_shop;

use actix::prelude::*;
use ice_cream_shop::{AddIceCream, AddOrder, IceCreamShop};
use serde_json::from_slice;
use shared::order::Order;
use tokio::io::AsyncBufReadExt;
use tokio::io::BufReader;
use tokio::net::TcpListener;

#[actix_rt::main]
async fn main() {
    // Crear el IceCreamShop
    let ice_cream_shop = IceCreamShop::new().start();

    // Agregar algunos helados iniciales
    let res = ice_cream_shop
        .send(AddIceCream {
            flavor: "Vanilla".to_string(),
            quantity: 10,
        })
        .await;

    match res {
        Ok(_) => println!("Helado de vainilla agregado exitosamente"),
        Err(_) => println!("Falló al agregar helado de vainilla"),
    }

    let res = ice_cream_shop
        .send(AddIceCream {
            flavor: "Chocolate".to_string(),
            quantity: 20,
        })
        .await;

    match res {
        Ok(_) => println!("Helado de chocolate agregado exitosamente"),
        Err(_) => println!("Falló al agregar helado de chocolate"),
    }

    // Iniciar el servidor TCP
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();

    println!("Esperando conexiones!");

    loop {
        let (stream, addr) = listener.accept().await.unwrap();
        println!("[{:?}] Cliente conectado", addr);

        let mut reader = BufReader::new(stream);

        let mut line = String::new();
        while reader.read_line(&mut line).await.unwrap() > 0 {
            match from_slice::<Order>(line.trim().as_bytes()) {
                Ok(order) => {
                    println!(
                        "[{:?}] Received order: {} of {}",
                        addr, order.quantity, order.flavor
                    );

                    let res = ice_cream_shop.send(AddOrder { order }).await;
                    match res {
                        Ok(_) => println!("[{:?}] Order enqueue successfully \n", addr),
                        Err(_) => println!("[{:?}] Failed to process order \n", addr),
                    }
                }
                Err(e) => {
                    println!("[{:?}] Failed to deserialize order: {}", addr, e);
                }
            }
            line.clear();
        }
    }
}
