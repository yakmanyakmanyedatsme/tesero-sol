use serde::{Deserialize, Serialize};
#[derive(Serialize, Clone, Deserialize, PartialEq, Debug)]
pub struct Address {
    pub street_address: String,
    pub city: String,
    pub state_province: String,
    pub zip_postal_code: String,
    pub country: String,
}
