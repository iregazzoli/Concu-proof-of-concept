use crate::ice_cream_shop::{AddIceCream, IceCreamShop, RequestIceCream};
use actix::prelude::*;
#[actix_rt::main]
async fn main() {
    // Start the IceCreamShop actor
    let shop_addr = IceCreamShop::new().start();

    // Add a flavor to the shop
    let res = shop_addr
        .send(AddIceCream {
            flavor: "Vanilla".to_string(),
            quantity: 10,
        })
        .await;

    match res {
        Ok(_) => println!("Vanilla ice cream added successfully"),
        Err(_) => println!("Failed to add Vanilla ice cream"),
    }

    // Request a flavor that is in the shop
    let res = shop_addr
        .send(RequestIceCream {
            flavor: "Vanilla".to_string(),
            quantity: 5,
        })
        .await;

    match res {
        Ok(Ok(quantity)) => println!("Got {} scoops of Vanilla ice cream", quantity),
        Ok(Err(err)) => println!("Failed to get Vanilla ice cream: {}", err),
        Err(_) => println!("Failed to send request for Vanilla ice cream"),
    }

    // Request a flavor that is not in the shop
    let res = shop_addr
        .send(RequestIceCream {
            flavor: "Chocolate".to_string(),
            quantity: 5,
        })
        .await;

    match res {
        Ok(Ok(quantity)) => println!("Got {} scoops of Chocolate ice cream", quantity),
        Ok(Err(err)) => println!("Failed to get Chocolate ice cream: {}", err),
        Err(_) => println!("Failed to send request for Chocolate ice cream"),
    }

    // Request more ice cream than is available
    let res = shop_addr
        .send(RequestIceCream {
            flavor: "Vanilla".to_string(),
            quantity: 20,
        })
        .await;

    match res {
        Ok(Ok(quantity)) => println!("Got {} scoops of Vanilla ice cream", quantity),
        Ok(Err(err)) => println!("Failed to get Vanilla ice cream: {}", err),
        Err(_) => println!("Failed to send request for Vanilla ice cream"),
    }

    System::current().stop();
}
