// Copyright OxidOS Automotive 2024.

use crate::{peripherals::rng, Component};
use parse_macros::component;
use std::rc::Rc;

#[component(curr, ident = "rng")]
pub struct RngCapsule<R: rng::Rng + 'static> {
    _inner: Rc<R>,
}

impl<R: rng::Rng> Component for RngCapsule<R> {}
