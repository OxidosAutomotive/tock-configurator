// Copyright OxidOS Automotive 2024.

use std::rc::Rc;

use crate::menu::capsule_popup;
use crate::state::Data;
use parse::peripherals::{Chip, DefaultPeripherals};

/// Menu for configuring the timer capsule.
pub fn config<C: Chip + 'static + serde::Serialize, P: DefaultPeripherals>(
    chip: Rc<C>,
    previous_state: Option<
        Rc<<<C as parse::peripherals::Chip>::Peripherals as DefaultPeripherals>::Timer>,
    >,
) -> cursive::views::LinearLayout {
    match previous_state {
        None => config_unknown(chip),
        Some(inner) => match chip.peripherals().timer() {
            //  TODO: doc both arms.
            Ok(timer_peripherals) => {
                capsule_popup::<C, _>(crate::views::radio_group_with_null_known(
                    Vec::from(timer_peripherals),
                    on_timer_submit::<C>,
                    inner,
                ))
            }
            Err(_) => capsule_popup::<C, _>(crate::menu::no_support("TIMER")),
        },
    }
}

//  TODO: Rename and doc.
fn config_unknown<C: Chip + 'static + serde::ser::Serialize>(
    chip: Rc<C>,
) -> cursive::views::LinearLayout {
    match chip.peripherals().timer() {
        Ok(timer_peripherals) => capsule_popup::<C, _>(crate::views::radio_group_with_null(
            Vec::from(timer_peripherals),
            on_timer_submit::<C>,
        )),
        Err(_) => capsule_popup::<C, _>(crate::menu::no_support("TIMER")),
    }
}

/// Configure an alarm based on the submitted timer.
fn on_timer_submit<C: Chip + 'static + serde::ser::Serialize>(
    siv: &mut cursive::Cursive,
    submit: &Option<Rc<<C::Peripherals as DefaultPeripherals>::Timer>>,
) {
    if let Some(data) = siv.user_data::<Data<C>>() {
        match submit {
            Some(timer) => data.platform.update_alarm(timer.clone()),
            None => data.platform.remove_alarm(),
        }
    }
}
