// Copyright OxidOS Automotive 2024.

use std::rc::Rc;

use crate::menu::capsule_popup;
use crate::state::Data;
use parse::peripherals::{Chip, DefaultPeripherals};

/// Menu for configuring the Rng capsule.
pub fn config<C: Chip + 'static + serde::Serialize>(
    chip: Rc<C>,
    choice: Option<Rc<<<C as parse::peripherals::Chip>::Peripherals as DefaultPeripherals>::Rng>>,
) -> cursive::views::LinearLayout {
    match choice {
        None => config_unknown(chip),
        Some(inner) => match chip.peripherals().rng() {
            Ok(rng_peripherals) => {
                capsule_popup::<C, _>(crate::views::radio_group_with_null_known(
                    Vec::from(rng_peripherals),
                    on_rng_submit::<C>,
                    inner,
                ))
            }
            Err(_) => capsule_popup::<C, _>(crate::menu::no_support("RNG")),
        },
    }
}

/// Menu for configuring the Rng capsule.
fn config_unknown<C: Chip + 'static + serde::ser::Serialize>(
    chip: Rc<C>,
) -> cursive::views::LinearLayout {
    match chip.peripherals().rng() {
        Ok(rng_peripherals) => capsule_popup::<C, _>(crate::views::radio_group_with_null(
            Vec::from(rng_peripherals),
            on_rng_submit::<C>,
        )),
        Err(_) => capsule_popup::<C, _>(crate::menu::no_support("RNG")),
    }
}

/// Configure an Rng based on the submitted Rng.
fn on_rng_submit<C: Chip + 'static + serde::ser::Serialize>(
    siv: &mut cursive::Cursive,
    submit: &Option<Rc<<C::Peripherals as DefaultPeripherals>::Rng>>,
) {
    if let Some(data) = siv.user_data::<Data<C>>() {
        if let Some(rng) = submit {
            data.platform.update_rng(Rc::clone(rng));
        } else {
            data.platform.remove_rng();
        }
    }
}
