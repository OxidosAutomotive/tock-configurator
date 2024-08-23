// Copyright OxidOS Automotive 2024.

use parse::constants::PERIPHERALS;
use parse::peripheral;

#[derive(Debug)]
#[peripheral(serde, ident = "twi")]
pub struct MicroBitTwi {}

impl parse::I2c for MicroBitTwi {}
impl parse::Component for MicroBitTwi {}

impl std::fmt::Display for MicroBitTwi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "twi")
    }
}
