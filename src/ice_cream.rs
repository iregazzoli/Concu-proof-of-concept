use actix::prelude::*;

pub struct IceCream {
    quantity: u32,
}

impl IceCream {
    pub fn new(quantity: u32) -> Self {
        Self { quantity }
    }
}

impl Actor for IceCream {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        println!("IceCream started");
    }
}

#[derive(Message)]
#[rtype(result = "Result<u32, &'static str>")]
pub struct RequestFlavor {
    pub quantity: u32,
}

impl Handler<RequestFlavor> for IceCream {
    type Result = Result<u32, &'static str>;

    fn handle(&mut self, msg: RequestFlavor, _ctx: &mut Context<Self>) -> Self::Result {
        if self.quantity >= msg.quantity {
            self.quantity -= msg.quantity;
            Ok(msg.quantity)
        } else {
            Err("Not enough ice cream")
        }
    }
}
