
use serde::{Serialize, Deserialize};
use serde_json::Value,
use std::collections::HashMap;


/// ttn data struct for ttn payload
#[derive(Serialize, Deserialize, Debug)]
pub struct Payload {
    pub app_id: String,
    pub dev_id: String,
    pub hardware_serial: String,
    pub port: isize,
    pub counter: isize,
    pub payload_raw: String,
    pub payload_fiels: Value,
    pub metadata: Metadata,
    pub download_link: String,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Metadata {
    pub time: String,
    pub frequency: f64,
    pub modulation: String,
    pub data_rate: String,
    pub coding_rate: String,
    pub gateways: Vec<Gateway>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Gateway {
    pub gtw_id: String,
    pub gtw_trusted: bool,
    pub timestamp: usize,
    pub time: String,
    pub channel: isize,
    pub rssi: isize,
    pub snr: f64,
    pub rf_chain: isize,
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f64,
}