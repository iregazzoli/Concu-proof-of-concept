use crate::ice_cream::IceCream;
use actix::prelude::*;
use std::collections::HashMap;
use std::collections::VecDeque;

pub struct IceCreamShop {
    ice_creams: HashMap<String, Addr<IceCream>>,
    waiting_clients: VecDeque<String>,
    waiting_workers: VecDeque<String>,
}

impl IceCreamShop {
    #[allow(dead_code)]
    pub fn new() -> Self {
        IceCreamShop {
            ice_creams: HashMap::new(),
            waiting_clients: VecDeque::new(),
            waiting_workers: VecDeque::new(),
        }
    }
}

impl Actor for IceCreamShop {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        println!("IceCreamShop started");
    }
}

// ... Rest of the IceCream related code ...

#[derive(Message)]
#[rtype(result = "()")]
pub struct AddClient {
    pub client_id: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct AddWorker {
    pub worker_id: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct AddIceCream {
    pub flavor: String,
    pub quantity: u32,
}

impl Handler<AddIceCream> for IceCreamShop {
    type Result = ();

    fn handle(&mut self, msg: AddIceCream, _ctx: &mut Self::Context) {
        let ice_cream = IceCream::create(|_| IceCream::new(msg.quantity));
        self.ice_creams.insert(msg.flavor, ice_cream);
    }
}

impl Handler<AddClient> for IceCreamShop {
    type Result = ();

    fn handle(&mut self, msg: AddClient, _ctx: &mut Self::Context) {
        self.waiting_clients.push_back(msg.client_id);
        self.match_clients_and_workers();
    }
}

impl Handler<AddWorker> for IceCreamShop {
    type Result = ();

    fn handle(&mut self, msg: AddWorker, _ctx: &mut Self::Context) {
        self.waiting_workers.push_back(msg.worker_id);
        self.match_clients_and_workers();
    }
}

impl IceCreamShop {
    fn match_clients_and_workers(&mut self) {
        while let (Some(client_id), Some(worker_id)) = (
            self.waiting_clients.pop_front(),
            self.waiting_workers.pop_front(),
        ) {
            // Here you would need to send a message to the worker with the client_id
            // The worker would then establish a connection with the client using the client_id
            // The specifics of how this is done would depend on how you're managing network connections
        }
    }
}
