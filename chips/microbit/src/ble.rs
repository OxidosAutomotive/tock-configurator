// Copyright OxidOS Automotive 2024.

use parse::constants::PERIPHERALS;
use parse::peripheral;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub enum BleType {
    Ble,
}

#[derive(Debug)]
#[peripheral(serde, ident = "ble")]
pub struct MicroBitBle(BleType);

impl parse::Component for MicroBitBle {}
impl parse::BleAdvertisement for MicroBitBle {}

impl std::fmt::Display for MicroBitBle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MicroBitBle")
    }
}
