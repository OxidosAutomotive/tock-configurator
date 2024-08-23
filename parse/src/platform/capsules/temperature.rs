// Copyright OxidOS Automotive 2024.

use crate::{temp, Component};
use parse_macros::component;
use std::rc::Rc;

#[component(curr, ident = "temperature")]
pub struct Temperature<T: temp::Temperature + 'static> {
    /// Temperature driver used by the capsule.
    _inner: Rc<T>,
}

impl<T: temp::Temperature> Component for Temperature<T> {}
