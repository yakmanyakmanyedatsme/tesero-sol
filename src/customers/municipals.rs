use crate::general::address::Address;
use crate::general::contactinfo::ContactInfo;
use crate::general::credentials::Credentials;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Serialize, Clone, Deserialize, PartialEq, Debug)]
pub struct Municipal {
    pub cred: Credentials,
    pub name: String,
    pub contact: ContactInfo,
    pub address: Option<Address>,
    pub rep_position: Option<NaiveDate>,
}
