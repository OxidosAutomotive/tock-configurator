// Copyright OxidOS Automotive 2024.

use crate::gpio::MicroBitGpio;
use crate::{timer, uart, FlashType, MicroBitTwi, TemperatureType, UartType};
use parse::constants::PERIPHERALS;
use quote::{format_ident, quote};
use std::rc::Rc;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct MicroBitPeripherals {
    #[serde(skip)]
    ident: String,
    // Default peripherals for the Microbit.
    uart: [Rc<crate::uart::MicroBitUart>; 1],
    timer: [Rc<crate::timer::MicroBitTimer>; 1],
    ble: [Rc<crate::ble::MicroBitBle>; 1],
    rng: [Rc<crate::rng::MicroBitRng>; 1],
    temperature: [Rc<crate::temperature::MicroBitTemperature>; 1],
    twi: [Rc<crate::twi::MicroBitTwi>; 1],
    gpio: [Rc<crate::gpio::MicroBitGpio>; 1],
    flash: [Rc<crate::flash::MicroBitFlash>; 1],
}

impl MicroBitPeripherals {
    pub fn new() -> Self {
        Self {
            ident: PERIPHERALS.to_string(),
            uart: [Rc::new(uart::MicroBitUart::new(UartType::Uart0))],
            timer: [Rc::new(timer::MicroBitTimer::new(crate::TimerType::Rtc))],
            ble: [Rc::new(crate::ble::MicroBitBle::new(crate::BleType::Ble))],
            rng: [Rc::new(crate::rng::MicroBitRng::new(crate::RngType::Rng))],
            temperature: [Rc::new(crate::temperature::MicroBitTemperature::new(
                TemperatureType::Temp,
            ))],
            twi: [Rc::new(crate::MicroBitTwi::new())],
            flash: [Rc::new(crate::MicroBitFlash::new(FlashType::Flash0))],
            gpio: [Rc::new(crate::gpio::MicroBitGpio::new())],
        }
    }
}

impl Default for MicroBitPeripherals {
    fn default() -> Self {
        Self::new()
    }
}

impl parse::Component for MicroBitPeripherals {
    fn init_expr(&self) -> Result<parse::proc_macro2::TokenStream, parse::Error> {
        Ok(quote! {
             kernel::static_init!(
                 nrf52833::interrupt_service::Nrf52833DefaultPeripherals,
                 nrf52833::interrupt_service::Nrf52833DefaultPeripherals::new()
             )
        })
    }

    fn after_init(&self) -> Option<parse::proc_macro2::TokenStream> {
        let ident = format_ident!("{}", PERIPHERALS.clone());
        Some(quote! {
            #ident.init();
        })
    }
}

impl parse::Ident for MicroBitPeripherals {
    fn ident(&self) -> Result<&str, parse::error::Error> {
        Ok(&self.ident)
    }
}

impl parse::DefaultPeripherals for MicroBitPeripherals {
    type Gpio = MicroBitGpio;
    type Uart = crate::MicroBitUart;
    type Timer = crate::MicroBitTimer;
    type I2c = MicroBitTwi;
    type Spi = parse::NoSupport;
    type Rng = crate::MicroBitRng;
    type BleAdvertisement = crate::MicroBitBle;
    type Temperature = crate::MicroBitTemperature;
    type Flash = crate::MicroBitFlash;

    fn uart(&self) -> Result<&[Rc<Self::Uart>], parse::Error> {
        Ok(&self.uart)
    }

    fn timer(&self) -> Result<&[Rc<Self::Timer>], parse::Error> {
        Ok(&self.timer)
    }

    fn i2c(&self) -> Result<&[Rc<Self::I2c>], parse::Error> {
        Ok(&self.twi)
    }

    fn ble(&self) -> Result<&[Rc<Self::BleAdvertisement>], parse::Error> {
        Ok(&self.ble)
    }

    fn flash(&self) -> Result<&[Rc<Self::Flash>], parse::Error> {
        Ok(&self.flash)
    }

    fn temp(&self) -> Result<&[Rc<Self::Temperature>], parse::Error> {
        Ok(&self.temperature)
    }

    fn rng(&self) -> Result<&[Rc<Self::Rng>], parse::Error> {
        Ok(&self.rng)
    }

    fn gpio(&self) -> Result<&[Rc<Self::Gpio>], parse::Error> {
        Ok(&self.gpio)
    }
}
