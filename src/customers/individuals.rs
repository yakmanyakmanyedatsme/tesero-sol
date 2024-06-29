use crate::general::address::Address;
use crate::general::contactinfo::ContactInfo;
use crate::general::credentials::Credentials;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Clone, Deserialize, PartialEq, Debug)]
pub struct Individual {
    pub cred: Credentials,
    pub contact: ContactInfo,
    pub address: Option<Address>,
    pub sign_up_date: Option<NaiveDate>,
    pub maintenance: Option<Maintenance>,
    pub installations: Option<Installations>,
}

#[derive(Serialize, Clone, Deserialize, PartialEq, Debug)]
pub struct Program {
    pub program_type: ProgramType,
}

#[derive(Serialize, Clone, Deserialize, PartialEq, Debug)]
pub enum ProgramType {
    Battery,
    Solar,
    EVChargers,
    EnergyGenerationAndStorage,
}

#[derive(Serialize, Clone, Deserialize, PartialEq, Debug)]
pub struct Schedule {
    pub calendar: HashMap<NaiveDate, String>, // Example: mapping dates to events or tasks
    pub dashboard: String,                    // Placeholder for actual dashboard structure
}

#[derive(Serialize, Clone, Deserialize, PartialEq, Debug)]
pub struct Procurement {
    pub product_id: String,
    pub cost: f64,
    pub quantity: u32,
    pub dashboard: String, // Placeholder for actual dashboard structure
}

#[derive(Serialize, Clone, Deserialize, PartialEq, Debug)]
pub struct Maintenance {
    pub dashboard: String, // Placeholder for actual dashboard structure
}

#[derive(Serialize, Clone, Deserialize, PartialEq, Debug)]
pub struct Installations {
    pub dashboard: String, // Placeholder for actual dashboard structure
}
