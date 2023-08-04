#![allow(clippy::wrong_self_convention)]

mod sample;

use std::collections::{BTreeMap, HashMap};
use std::mem::take;
use rand::{Rng, RngCore, SeedableRng};
use rand::rngs::StdRng;

pub(crate) struct WeightedElement<T> {
    key: T,
    weight: f64,
    accumulated: f64,
}

pub struct WeightedList<T> {
    items: Vec<WeightedElement<T>>,
}

impl<T> WeightedList<T> {
    pub fn total_weight(&self) -> f64 {
        self.items.last().map_or(0.0, |elem| elem.accumulated)
    }

    pub fn merge(&mut self) where T: Ord {
        if self.items.is_empty() {
            return;
        }
        let mut new = BTreeMap::default();
        for item in take(&mut self.items) {
            new.entry(item.key).and_modify(|e: &mut f64| *e += item.weight).or_insert(item.weight);
        }
        self.items = WeightedList::from_iter(new).items;
    }

    pub fn shuffle(&self, mut rng: impl Rng) -> Vec<&T> {
        let mut order: Vec<_> = self
            .items
            .iter()
            .enumerate()
            .map(|(i, elem)| (i, -(rng.gen::<f64>().powf(1.0 / elem.weight as f64))))
            .collect();

        // Sort the elements based on the negative random power
        order.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        // Extract the shuffled items using the sorted order
        order.iter().map(|(i, _)| &self.items[*i].key).collect()
    }
}

impl<T> FromIterator<(T, f64)> for WeightedList<T> {
    fn from_iter<I: IntoIterator<Item=(T, f64)>>(iter: I) -> Self {
        let sequence = iter.into_iter();
        let mut accumulated = 0.0;
        let mut cumulative = Vec::with_capacity(sequence.size_hint().0);
        for (key, weight) in sequence {
            if weight >= 0.0 {
                accumulated += weight;
                cumulative.push(WeightedElement {
                    key,
                    weight,
                    accumulated,
                });
            } else {
                // nan, negative, zero, inf
            }
        }

        WeightedList {
            items: cumulative,
        }
    }
}
