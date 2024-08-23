// Copyright OxidOS Automotive 2024.

use std::rc::Rc;

use crate::menu::capsule_popup;
use crate::state::Data;
use parse::peripherals::{Chip, DefaultPeripherals};

/// Menu for configuring the ble radio capsule.
pub fn config<C: Chip + 'static + serde::Serialize>(
    chip: Rc<C>,
    choice: Option<(
        Rc<<C::Peripherals as DefaultPeripherals>::Timer>,
        Rc<<C::Peripherals as DefaultPeripherals>::BleAdvertisement>,
    )>,
) -> cursive::views::LinearLayout {
    match choice {
        None => config_unknown(chip),
        Some(inner) => {
            let inner_ble = inner.1;
            match chip.peripherals().ble() {
                Ok(ble_peripherals) => {
                    capsule_popup::<C, _>(crate::views::radio_group_with_null_known(
                        Vec::from(ble_peripherals),
                        move |siv, submit| {
                            on_ble_submit::<C>(chip.clone(), siv, submit, Some(inner.0.clone()))
                        },
                        inner_ble,
                    ))
                }
                Err(_) => capsule_popup::<C, _>(crate::menu::no_support("BLE")),
            }
        }
    }
}

fn config_unknown<C: Chip + 'static + serde::ser::Serialize>(
    chip: Rc<C>,
) -> cursive::views::LinearLayout {
    match chip.peripherals().ble() {
        Ok(ble_peripherals) => capsule_popup::<C, _>(crate::views::radio_group_with_null(
            Vec::from(ble_peripherals),
            move |siv, submit| on_ble_submit::<C>(chip.clone(), siv, submit, None),
        )),
        Err(_) => capsule_popup::<C, _>(crate::menu::no_support("BLE")),
    }
}

/// Configure an alarm based on the submitted ble.
fn on_ble_submit<C: Chip + 'static + serde::ser::Serialize>(
    chip: Rc<C>,
    siv: &mut cursive::Cursive,
    submit: &Option<Rc<<C::Peripherals as DefaultPeripherals>::BleAdvertisement>>,
    previous_timer: Option<
        Rc<<<C as parse::peripherals::Chip>::Peripherals as DefaultPeripherals>::Timer>,
    >,
) {
    siv.pop_layer();
    if let Some(data) = siv.user_data::<Data<C>>() {
        if let Some(ble) = submit {
            siv.add_layer(timer_popup::<C>(chip, ble.clone(), previous_timer))
        } else {
            data.platform.remove_ble();
        }
    }
}

fn timer_popup<C: Chip + 'static + serde::ser::Serialize>(
    chip: Rc<C>,
    submit: Rc<<C::Peripherals as DefaultPeripherals>::BleAdvertisement>,
    previous_timer: Option<
        Rc<<<C as parse::peripherals::Chip>::Peripherals as DefaultPeripherals>::Timer>,
    >,
) -> cursive::views::LinearLayout {
    match previous_timer {
        Some(prev) => {
            let inner = prev;
            match chip.peripherals().timer() {
                Ok(timer_peripherals) => {
                    capsule_popup::<C, _>(crate::views::radio_group_with_null_known(
                        Vec::from(timer_peripherals),
                        move |siv, submit_timer| {
                            on_timer_submit::<C>(siv, submit_timer, submit.clone())
                        },
                        inner,
                    ))
                }
                Err(_) => capsule_popup::<C, _>(crate::menu::no_support("TIMER")),
            }
        }
        None => match chip.peripherals().timer() {
            Ok(timer_peripherals) => capsule_popup::<C, _>(crate::views::radio_group_with_null(
                Vec::from(timer_peripherals),
                move |siv, submit_timer| on_timer_submit::<C>(siv, submit_timer, submit.clone()),
            )),
            Err(_) => capsule_popup::<C, _>(crate::menu::no_support("TIMER")),
        },
    }
}

fn on_timer_submit<C: Chip + 'static + serde::ser::Serialize>(
    siv: &mut cursive::Cursive,
    submit_timer: &Option<Rc<<C::Peripherals as DefaultPeripherals>::Timer>>,
    submit_ble: Rc<<C::Peripherals as DefaultPeripherals>::BleAdvertisement>,
) {
    if let Some(data) = siv.user_data::<Data<C>>() {
        if let Some(timer) = submit_timer {
            data.platform.update_ble(submit_ble, timer.clone());
        } else {
            data.platform.remove_ble();
        }
    }
}
