// Copyright OxidOS Automotive 2024.

use std::error::Error;
use std::rc::Rc;

use crate::config::{Capsule, Configuration};
use crate::{
    AlarmDriver, Chip, Console, DefaultPeripherals, MuxAlarm, MuxSpi, MuxUart, Platform, Scheduler,
    SpiController,
};

/// The context provided for Tock's `main` file.
///
/// This should be created from a [`Configuration`], as it's meant to be the glue between
/// the user's agnostic configuration and the Tock's specific internals needed for the code generation
/// process.
pub struct Context<C: Chip> {
    //  TODO: Doc this
    pub platform: Rc<Platform<C>>,
    pub chip: Rc<C>,
    pub process_count: usize,
    pub stack_size: usize,
}

impl<C: Chip> Context<C> {
    pub fn from_config(
        chip: C,
        config: Configuration<C::Peripherals>,
    ) -> Result<Self, Box<dyn Error>> {
        let mut visited = Vec::new();
        let mut capsules = Vec::new();

        // Iterate over the capsules and insert them into the current platform's capsule list.
        for capsule_config in config.capsules() {
            match capsule_config {
                Capsule::Console { uart, baud_rate } => {
                    let mut uuart = uart.clone();
                    for uart_with_ident in chip.peripherals().as_ref().uart().unwrap() {
                        if uart_with_ident == uart {
                            uuart = uart_with_ident.clone()
                        }
                    }
                    let mux_uart = MuxUart::insert_get(uuart.clone(), *baud_rate, &mut visited);
                    capsules.push(Console::get(mux_uart) as Rc<dyn crate::Capsule>)
                }
                Capsule::Alarm { timer } => {
                    let mut utimer = timer.clone();
                    for timer_with_ident in chip.peripherals().as_ref().timer().unwrap() {
                        if timer_with_ident == timer {
                            utimer = timer_with_ident.clone()
                        }
                    }
                    let mux_alarm = MuxAlarm::insert_get(utimer.clone(), &mut visited);
                    capsules.push(AlarmDriver::get(mux_alarm) as Rc<dyn crate::Capsule>)
                }
                Capsule::Spi { spi } => {
                    let mux_spi = MuxSpi::insert_get(spi.clone(), &mut visited);
                    capsules.push(SpiController::get(mux_spi) as Rc<dyn crate::Capsule>)
                }
                _ => {}
            };
        }

        //  TODO: Act with the chip same as with virtualizers? Make sure the Rcs of the peripherals
        // are pointing to the same instance.
        Ok(Self {
            platform: Rc::new(Platform::<C>::new(
                config.r#type,
                capsules,
                Scheduler::insert_get(config.scheduler, &mut visited),
                chip.systick()?,
            )),
            chip: Rc::new(chip),
            process_count: config.process_count,
            stack_size: config.stack_size.into(),
        })
    }
}
