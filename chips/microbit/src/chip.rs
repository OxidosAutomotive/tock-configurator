// Copyright OxidOS Automotive 2024.

use crate::MicroBitPeripherals;
use common::cortex_m4::Systick;
use parse::constants::CHIP;
use parse::peripherals::chip::Chip;
use parse::{Component, Ident};
use quote::{format_ident, quote};
use std::rc::Rc;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct MicroBitChip {
    #[serde(skip)]
    ident: String,
    #[serde(skip)]
    systick: Rc<Systick>,

    peripherals: Rc<MicroBitPeripherals>,
}

impl Default for MicroBitChip {
    fn default() -> Self {
        Self {
            ident: CHIP.to_string(),
            peripherals: Rc::new(MicroBitPeripherals::new()),
            systick: Rc::new(Systick::new()),
        }
    }
}

impl MicroBitChip {
    pub fn new() -> Self {
        Self::default()
    }
}
impl Ident for MicroBitChip {
    fn ident(&self) -> Result<&str, parse::error::Error> {
        Ok(&self.ident)
    }
}

impl Component for MicroBitChip {
    fn ty(&self) -> Result<parse::proc_macro2::TokenStream, parse::Error> {
        Ok(quote!(
            nrf52833::chip::NRF52<
                'static,
                nrf52833::interrupt_service::Nrf52833DefaultPeripherals<'static>,
            >
        ))
    }

    fn dependencies(&self) -> Option<Vec<Rc<dyn Component>>> {
        Some(vec![self.peripherals.clone()])
    }

    fn init_expr(&self) -> Result<parse::proc_macro2::TokenStream, parse::Error> {
        let peripherals_ident = format_ident!("{}", self.peripherals.ident()?);
        Ok(quote! {
            kernel::static_init!(
                nrf52833::chip::NRF52<nrf52833::interrupt_service::Nrf52833DefaultPeripherals>,
                nrf52833::chip::NRF52::new(#peripherals_ident)
            );
        })
    }

    fn after_init(&self) -> Option<parse::proc_macro2::TokenStream> {
        let ident = format_ident!("{}", self.ident);
        Some(quote!(CHIP = Some(#ident);))
    }
}

impl Chip for MicroBitChip {
    type Peripherals = MicroBitPeripherals;
    type Systick = Systick;

    fn peripherals(&self) -> Rc<Self::Peripherals> {
        self.peripherals.clone()
    }

    fn systick(&self) -> Result<Rc<Self::Systick>, parse::Error> {
        Ok(self.systick.clone())
    }
}
