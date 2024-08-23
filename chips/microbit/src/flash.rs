// Copyright OxidOS Automotive 2024.

use parse::constants::PERIPHERALS;
use parse::peripheral;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub enum FlashType {
    Flash0,
}

#[derive(Debug)]
#[peripheral(serde, ident = "flash")]
pub struct MicroBitFlash(FlashType);

impl parse::Component for MicroBitFlash {}
impl parse::Flash for MicroBitFlash {}

impl std::fmt::Display for MicroBitFlash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "flash")
    }
}
