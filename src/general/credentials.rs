use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Clone, Deserialize, PartialEq, Debug)]
pub struct Credentials {
    pub email: String,
    pub password: String,
}
