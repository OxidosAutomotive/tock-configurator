// Copyright OxidOS Automotive 2024.

use std::rc::Rc;

use crate::menu::capsule_popup;
use crate::state::Data;
use parse::peripherals::{Chip, DefaultPeripherals};

/// Menu for configuring the I2C capsule.
pub fn config<C: Chip + 'static + serde::Serialize>(
    chip: Rc<C>,
    choice: Option<Rc<<<C as parse::peripherals::Chip>::Peripherals as DefaultPeripherals>::I2c>>,
) -> cursive::views::LinearLayout {
    match choice {
        None => config_unknown(chip),
        Some(inner) => match chip.peripherals().i2c() {
            Ok(i2c_peripherals) => {
                capsule_popup::<C, _>(crate::views::radio_group_with_null_known(
                    Vec::from(i2c_peripherals),
                    on_i2c_submit::<C>,
                    inner,
                ))
            }
            Err(_) => capsule_popup::<C, _>(crate::menu::no_support("I2C")),
        },
    }
}

fn config_unknown<C: Chip + 'static + serde::ser::Serialize>(
    chip: Rc<C>,
) -> cursive::views::LinearLayout {
    match chip.peripherals().i2c() {
        Ok(i2c_peripherals) => capsule_popup::<C, _>(crate::views::radio_group_with_null(
            Vec::from(i2c_peripherals),
            on_i2c_submit::<C>,
        )),
        Err(_) => capsule_popup::<C, _>(crate::menu::no_support("I2C")),
    }
}

/// Configure an I2C master based on the submitted I2C.
fn on_i2c_submit<C: Chip + 'static + serde::ser::Serialize>(
    siv: &mut cursive::Cursive,
    submit: &Option<Rc<<C::Peripherals as DefaultPeripherals>::I2c>>,
) {
    if let Some(data) = siv.user_data::<Data<C>>() {
        if let Some(i2c) = submit {
            data.platform.update_i2c(i2c.clone());
        } else {
            data.platform.remove_i2c();
        }
    }
}
