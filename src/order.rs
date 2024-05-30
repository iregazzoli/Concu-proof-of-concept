use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Order {
    pub flavor: String,
    pub quantity: u32,
    pub id: Option<u32>,
}
