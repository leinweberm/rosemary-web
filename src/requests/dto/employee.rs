use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Employee {
    pub rate: u32,
    pub name: String,
}