use crate::ice_cream::{IceCream, RequestFlavor};
use crate::order::Order;
use actix::prelude::*;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use tokio::net::TcpStream;

use tokio::io::AsyncWriteExt;

pub struct ClientActor {
    stream: Arc<Mutex<TcpStream>>,
}

impl Actor for ClientActor {
    type Context = Context<Self>;
}

impl ClientActor {
    pub fn new(stream: Arc<Mutex<TcpStream>>) -> Self {
        ClientActor { stream }
    }
}

pub struct SendMessageToClient {
    pub message: String,
}

impl Message for SendMessageToClient {
    type Result = Result<(), std::io::Error>;
}

impl Handler<SendMessageToClient> for ClientActor {
    type Result = ResponseActFuture<Self, Result<(), std::io::Error>>;

    fn handle(&mut self, msg: SendMessageToClient, _: &mut Self::Context) -> Self::Result {
        let stream = Arc::clone(&self.stream);
        let message = msg.message;
        Box::pin(
            async move {
                let mut stream = stream.lock().unwrap();
                stream.write_all(message.as_bytes()).await
            }
            .into_actor(self),
        )
    }
}

pub struct IceCreamShop {
    ice_creams: HashMap<String, Addr<IceCream>>,
    orders: VecDeque<Order>,
    clients: HashMap<u32, Addr<ClientActor>>,
}

impl IceCreamShop {
    pub fn new() -> Self {
        IceCreamShop {
            ice_creams: HashMap::new(),
            orders: VecDeque::new(),
            clients: HashMap::new(),
        }
    }
}

impl Actor for IceCreamShop {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        println!("IceCreamShop started");
    }
}

#[derive(Message)]
#[rtype(result = "Result<u32, &'static str>")]
pub struct RequestIceCream {
    pub flavor: String,
    pub quantity: u32,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct AddIceCream {
    pub flavor: String,
    pub quantity: u32,
}

impl Handler<RequestIceCream> for IceCreamShop {
    type Result = ResponseFuture<Result<u32, &'static str>>;

    fn handle(&mut self, msg: RequestIceCream, _ctx: &mut Self::Context) -> Self::Result {
        if let Some(ice_cream) = self.ice_creams.get(&msg.flavor) {
            let ice_cream = ice_cream.clone();
            let res = async move {
                match ice_cream
                    .send(RequestFlavor {
                        quantity: msg.quantity,
                    })
                    .await
                {
                    Ok(Ok(quantity)) => Ok(quantity),
                    Ok(Err(_)) => Err("Ice cream flavor is out of stock"),
                    Err(_) => Err("Error occurred while processing the request"),
                }
            };
            Box::pin(res)
        } else {
            Box::pin(async { Err("Flavor not found") })
        }
    }
}

impl Handler<AddIceCream> for IceCreamShop {
    type Result = ();

    fn handle(&mut self, msg: AddIceCream, _ctx: &mut Self::Context) {
        let ice_cream = IceCream::create(|_| IceCream::new(msg.quantity));
        self.ice_creams.insert(msg.flavor, ice_cream);
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct AddOrder {
    pub order: Order,
}

#[derive(Message)]
#[rtype(result = "Option<Order>")]
pub struct RemoveOrder;

impl Handler<AddOrder> for IceCreamShop {
    type Result = ();

    fn handle(&mut self, msg: AddOrder, _ctx: &mut Self::Context) {
        self.orders.push_back(msg.order);
    }
}

impl Handler<RemoveOrder> for IceCreamShop {
    type Result = Option<Order>;

    fn handle(&mut self, _msg: RemoveOrder, _ctx: &mut Self::Context) -> Self::Result {
        self.orders.pop_front()
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct AddClient {
    pub id: u32,
    pub actor: Addr<ClientActor>,
}

impl Handler<AddClient> for IceCreamShop {
    type Result = ();

    fn handle(&mut self, msg: AddClient, _ctx: &mut Self::Context) {
        self.clients.insert(msg.id, msg.actor);
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct SendClientMessage {
    pub id: u32,
    pub message: String,
}

impl Handler<SendClientMessage> for IceCreamShop {
    type Result = ();

    fn handle(&mut self, msg: SendClientMessage, _ctx: &mut Self::Context) {
        if let Some(client) = self.clients.get(&msg.id) {
            let _ = client.do_send(SendMessageToClient {
                message: msg.message,
            });
        } else {
            println!("Failed to send message to client: client not found");
        }
    }
}
