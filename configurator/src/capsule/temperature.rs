// Copyright OxidOS Automotive 2024.

use std::rc::Rc;

use crate::menu::capsule_popup;
use crate::state::Data;
use parse::peripherals::{Chip, DefaultPeripherals};

/// Menu for configuring the Temperature capsule.
pub fn config<C: Chip + 'static + serde::Serialize>(
    chip: Rc<C>,
    choice: Option<
        Rc<<<C as parse::peripherals::Chip>::Peripherals as DefaultPeripherals>::Temperature>,
    >,
) -> cursive::views::LinearLayout {
    match choice {
        None => config_unknown(chip),
        Some(inner) => match chip.peripherals().temp() {
            Ok(temp_peripherals) => {
                capsule_popup::<C, _>(crate::views::radio_group_with_null_known(
                    Vec::from(temp_peripherals),
                    on_temp_submit::<C>,
                    inner,
                ))
            }
            Err(_) => capsule_popup::<C, _>(crate::menu::no_support("TEMPERATURE")),
        },
    }
}

fn config_unknown<C: Chip + 'static + serde::ser::Serialize>(
    chip: Rc<C>,
) -> cursive::views::LinearLayout {
    match chip.peripherals().temp() {
        Ok(temp_peripherals) => capsule_popup::<C, _>(crate::views::radio_group_with_null(
            Vec::from(temp_peripherals),
            on_temp_submit::<C>,
        )),
        Err(_) => capsule_popup::<C, _>(crate::menu::no_support("TEMPERATURE")),
    }
}

/// Configure an Temperature based on the submitted Temperature.
fn on_temp_submit<C: Chip + 'static + serde::ser::Serialize>(
    siv: &mut cursive::Cursive,
    submit: &Option<Rc<<C::Peripherals as DefaultPeripherals>::Temperature>>,
) {
    if let Some(data) = siv.user_data::<Data<C>>() {
        if let Some(temp) = submit {
            data.platform.update_temp(Rc::clone(temp));
        } else {
            data.platform.remove_temp();
        }
    }
}
