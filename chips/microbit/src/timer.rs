// Copyright OxidOS Automotive 2024.

use parse::constants::PERIPHERALS;
use parse::{peripheral, Component};
use quote::quote;

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
pub enum TimerType {
    Rtc,
}

#[derive(Debug)]
#[peripheral(serde, ident = ".nrf52.rtc")]
pub struct MicroBitTimer(TimerType);

impl Component for MicroBitTimer {
    fn ty(&self) -> Result<parse::proc_macro2::TokenStream, parse::Error> {
        Ok(quote!(nrf52::rtc::Rtc<'static>))
    }
}

//  FIXME: This should be removed in the next iteration.
impl PartialEq for MicroBitTimer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl parse::Timer for MicroBitTimer {
    fn frequency(&self) -> usize {
        0
    }
}

impl std::fmt::Display for MicroBitTimer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "rtc")
    }
}
