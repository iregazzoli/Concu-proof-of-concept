use shared::order::Order;
use std::fs::File;
use std::io::BufReader; // Importar Order desde la biblioteca compartida

pub struct Client {
    orders: Vec<Order>,
}

impl Client {
    pub fn new(orders: Vec<Order>) -> Self {
        Self { orders }
    }

    pub fn place_order(&mut self) -> Option<Order> {
        self.orders.pop()
    }

    pub fn load_orders_from_file(file_path: &str) -> Vec<Order> {
        let file = File::open(file_path).expect("Unable to open file");
        let reader = BufReader::new(file);
        serde_json::from_reader(reader).expect("Unable to parse JSON")
    }
}
