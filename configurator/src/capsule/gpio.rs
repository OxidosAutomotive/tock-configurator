// Copyright OxidOS Automotive 2024.

use crate::items::ToMenuItem;
use crate::menu::{capsule_popup, checkbox_popup, no_support, pin_list_disabled};
use crate::state::{on_exit_submit, on_quit_submit, Data, GpioMap, PinFunction};
use crate::views;
use cursive::views::{Checkbox, ListChild, ListView};
use parse::peripherals::{Chip, DefaultPeripherals, Gpio};
use std::rc::Rc;

use super::ConfigMenu;

#[derive(Debug)]
pub(crate) struct GpioConfig;

impl ConfigMenu for GpioConfig {
    //  TODO: doc this.
    fn config<C: Chip + 'static + serde::ser::Serialize>(
        chip: Rc<C>,
    ) -> cursive::views::LinearLayout {
        match chip.peripherals().gpio() {
            //  TODO: Doc both arms.
            Ok(list) => capsule_popup::<C, _>(views::select_menu(
                Vec::from(list)
                    .into_iter()
                    .map(|elem| elem.to_menu_item())
                    .collect(),
                |siv, submit| on_gpio_capsule_submit::<C>(siv, Rc::clone(submit)),
            )),
            Err(_) => capsule_popup::<C, _>(no_support("GPIO")),
        }
    }
}

//  TODO: doc this.
fn gpio_pins_popup<C: Chip + 'static + serde::ser::Serialize>(
    gpio: Rc<<C::Peripherals as DefaultPeripherals>::Gpio>,
    pin_list: GpioMap<C>,
) -> cursive::views::LinearLayout {
    let view = pin_list_disabled::<C>(pin_list, PinFunction::Gpio, "gpio_pins");
    let gpio_clone = Rc::clone(&gpio);
    checkbox_popup::<_, _, _>(
        view,
        move |siv| on_gpio_pin_submit::<C>(siv, false, Rc::clone(&gpio)),
        move |siv| on_gpio_pin_submit::<C>(siv, true, gpio_clone.clone()),
    )
}

//  TODO: doc this.
fn on_gpio_capsule_submit<C: Chip + 'static + serde::Serialize>(
    siv: &mut cursive::Cursive,
    submit: Rc<<<C as Chip>::Peripherals as DefaultPeripherals>::Gpio>,
) {
    siv.pop_layer();
    if let Some(data) = siv.user_data::<Data<C>>() {
        // This never panics.
        let pin_list = data.gpio(&submit).unwrap().pins().clone();

        siv.add_layer(gpio_pins_popup::<C>(submit, pin_list));
    }
}

//  TODO: doc this.
fn on_gpio_pin_submit<C: Chip + 'static + serde::Serialize>(
    siv: &mut cursive::Cursive,
    quit: bool,
    gpio: Rc<<C::Peripherals as DefaultPeripherals>::Gpio>,
) {
    // Get the selected pins' labels from the configuration menu.
    let mut selected_pins_labels = Vec::new();

    // Retrieve the selected pins from the gpio pins list.
    siv.call_on_name("gpio_pins", |list_view: &mut ListView| {
        list_view.children().iter().for_each(|child| {
            if let ListChild::Row(label, view) = child {
                view.downcast_ref::<Checkbox>().map(|c| {
                    c.is_checked()
                        .then(|| selected_pins_labels.push(label.clone()))
                });
            };
        });
    });

    if let Some(data) = siv.user_data::<Data<C>>() {
        // The newly selected pins and the newly removed pins by the user.
        let mut selected_pins = Vec::new();

        if let Some(pins) = gpio.pins() {
            pins.as_ref().iter().for_each(|pin| {
                // Convert from label to PinId.
                selected_pins_labels
                    .contains(&format!("{}", pin))
                    .then(|| selected_pins.push(*pin));
            });
        }

        let mut unselected_pins = Vec::new();
        for (pin, pin_function) in data.gpio(&gpio).unwrap().pins() {
            if *pin_function == PinFunction::Gpio && !selected_pins.contains(pin) {
                unselected_pins.push(*pin);
            }
        }

        unselected_pins.iter().for_each(|pin| {
            data.change_pin_status(gpio.clone(), *pin, PinFunction::None);
        });

        selected_pins.iter().for_each(|pin| {
            data.change_pin_status(gpio.clone(), *pin, PinFunction::Gpio);
        });

        //panic!("{:?}", selected_pins);

        if selected_pins.is_empty() {
            data.platform.remove_gpio();
        } else {
            data.platform.update_gpio(selected_pins);
        }
    }

    if quit {
        on_quit_submit::<C>(siv);
    } else {
        on_exit_submit::<C>(siv);
    }
}
