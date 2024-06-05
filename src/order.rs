pub struct Order {
    flavor: String,
    quantity: u32,
    client_id: u32,
}

impl Order {
    pub fn new(flavor: String, quantity: u32, client_id: u32) -> Self {
        Self {
            flavor,
            quantity,
            client_id,
        }
    }

    pub fn flavor(&self) -> &String {
        &self.flavor
    }

    pub fn quantity(&self) -> u32 {
        self.quantity
    }

    pub fn client_id(&self) -> u32 {
        self.client_id
    }
}