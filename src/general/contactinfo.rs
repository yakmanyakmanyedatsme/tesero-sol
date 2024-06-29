use serde::{Deserialize, Serialize};
#[derive(Serialize, Clone, Deserialize, PartialEq, Debug)]
pub struct ContactInfo {
    pub first: String,
    pub last: String,
    pub middle_name: Option<String>,
    pub phone_number: String,
    pub email: String,
}
