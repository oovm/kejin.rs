#![allow(clippy::wrong_self_convention)]

mod sample;

use rand::RngCore;

pub(crate) struct WeightedElement<T> {
    key: T,
    weight: usize,
    accumulated: usize,
}

pub struct WeightedList<T> {
    items: Vec<T>,
    cumulative: Vec<T>,
}

pub struct WeightedDict<T> {
    items: Vec<T>,
    cumulative: Vec<T>,
}

impl<T> WeightedList<T> {
    pub fn as_dict(self) {}
    pub fn random(&self, rand: impl RngCore) -> Option<T> {

    }
}

impl<T> WeightedDict<T> {
    pub fn as_list(self) {}
}