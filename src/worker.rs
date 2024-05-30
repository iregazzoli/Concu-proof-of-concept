use serde_json::from_slice;
use shared::order::Order;
use std::io::prelude::*;
use std::net::TcpStream;
use std::str::from_utf8;
pub struct IceCreamWorker {
    id: usize,
}

impl IceCreamWorker {
    pub fn new(id: usize) -> Self {
        IceCreamWorker { id }
    }
}

pub struct ProcessOrder {
    pub order: Order,
}

impl Message for ProcessOrder {
    type Result = ();
}

impl Actor for IceCreamWorker {
    type Context = Context<Self>;
}

impl Handler<ProcessOrder> for IceCreamWorker {
    type Result = ResponseActFuture<Self, ()>;

    fn handle(&mut self, msg: ProcessOrder, _ctx: &mut Context<Self>) -> Self::Result {
        println!(
            "Worker {} processing order: {} of {}",
            self.id, msg.order.quantity, msg.order.flavor
        );

        let sleep_duration = Duration::from_secs(msg.order.quantity as u64);
        Box::pin(
            async move {
                sleep(sleep_duration).await;
                println!(
                    "Worker {} finished processing order: {} of {}",
                    self.id, msg.order.quantity, msg.order.flavor
                );
            }
            .into_actor(self),
        )
    }
}
