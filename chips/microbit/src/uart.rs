// Copyright OxidOS Automotive 2024.

use parse::{constants::PERIPHERALS, peripheral};

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
pub enum UartType {
    Uart0,
}

#[derive(Debug)]
#[peripheral(serde, ident = ".nrf52.uarte0")]
pub struct MicroBitUart(UartType);

impl Default for MicroBitUart {
    fn default() -> Self {
        Self::new(UartType::Uart0)
    }
}

impl parse::Component for MicroBitUart {}

//  FIXME: This should be removed in the next iteration.
impl PartialEq for MicroBitUart {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl parse::Uart for MicroBitUart {}
impl std::fmt::Display for MicroBitUart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "uarte0")
    }
}
