pub struct Order {
    flavor: String,
    quantity: u32,
    tx: tokio::sync::mpsc::Sender<String>,
}

impl Order {
    pub fn new(flavor: String, quantity: u32, tx: tokio::sync::mpsc::Sender<String>) -> Self {
        Self {
            flavor,
            quantity,
            tx,
        }
    }

    pub fn flavor(&self) -> &String {
        &self.flavor
    }

    pub fn quantity(&self) -> u32 {
        self.quantity
    }

    pub fn tx(&self) -> &tokio::sync::mpsc::Sender<String> {
        &self.tx
    }
}
