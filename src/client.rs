use std::fs::File;

pub struct Client {
    orders: Vec<(String, u32)>,
}

impl Client {
    pub fn new(orders: Vec<(String, u32)>) -> Self {
        Self { orders }
    }

    pub fn place_order(&mut self) -> Option<(String, u32)> {
        self.orders.pop()
    }

    pub fn load_orders_from_file(file_path: &str) -> Vec<(String, u32)> {
        let file = File::open(file_path).expect("Unable to open file");
        let orders: Vec<(String, u32)> =
            serde_yaml::from_reader(file).expect("Unable to parse YAML");
        orders
    }
}
