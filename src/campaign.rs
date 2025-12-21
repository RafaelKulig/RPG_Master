use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Campaign{
    pub name: String
}