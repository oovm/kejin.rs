#![allow(clippy::wrong_self_convention)]

mod builder;
mod display;
mod sample;

use rand::Rng;
use std::{collections::BTreeMap, mem::take, ops::AddAssign, slice::Iter};

#[derive(Debug)]
pub struct WeightedElement<T> {
    pub key: T,
    pub weight: f64,
    pub accumulated: f64,
}

/// Standard gacha model, each card has a weighted value, and the shipping probability is determined according to the weighted value.
#[derive(Debug)]
pub struct WeightedList<T> {
    items: Vec<WeightedElement<T>>,
    total: f64,
}

impl<T> WeightedList<T> {
    /// Get the total weight of all elements
    pub fn total_weight(&self) -> f64 {
        self.total
    }
    /// Reorganize and merge elements with the same name to speed up sampling
    pub fn merge(&mut self)
    where
        T: Ord,
    {
        if self.items.is_empty() {
            return;
        }
        let mut new = BTreeMap::default();
        for item in take(&mut self.items) {
            new.entry(item.key).and_modify(|e: &mut f64| *e += item.weight).or_insert(item.weight);
        }
        self.items = WeightedList::from_iter(new).items;
    }
    /// Randomly select an element based on weight, the higher the weight, the easier it is to be selected.
    pub fn random(&self, mut rng: impl Rng) -> Option<&T> {
        if self.items.is_empty() {
            return None;
        }
        let total_weight = self.total_weight();
        let random_weight = rng.gen_range(0.0..total_weight);
        let index = self
            .items
            .binary_search_by(|elem| elem.accumulated.total_cmp(&random_weight)) //
            .unwrap_or_else(|i| i);
        Some(&self.items[index].key)
    }
    /// Rearrange all elements according to weight, the higher the weight, the higher the front.
    pub fn shuffle(&self, mut rng: impl Rng) -> Vec<&T> {
        if self.items.is_empty() {
            return vec![];
        }
        let mut order: Vec<_> = self
            .items
            .iter()
            .enumerate() //
            .map(|(i, e)| (i, -(rng.gen::<f64>().powf(1.0 / e.weight))))
            .collect();
        // Sort the elements based on the negative random power
        order.sort_by(|a, b| a.1.total_cmp(&b.1));
        // Extract the shuffled items using the sorted order
        order.iter().map(|(i, _)| &self.items[*i].key).collect()
    }
    /// Select n elements according to the weight, the higher the weight, the easier it is to be selected
    pub fn random_select(&self, count: usize, mut rng: impl Rng) -> Vec<&T> {
        match self.items.len() {
            0 => return vec![],
            1 => return vec![&self.items[0].key; count],
            _ => {
                let mut out = Vec::with_capacity(count);
                for _ in 0..count {
                    if let Some(item) = self.random(&mut rng) {
                        out.push(item);
                    }
                }
                out
            }
        }
    }
    /// Select n unique elements according to the weight, the higher the weight, the higher the front
    pub fn random_sample(&self, count: usize, mut rng: impl Rng) -> Vec<&T> {
        let mut shuffle = self.shuffle(&mut rng);
        shuffle.truncate(count);
        shuffle
    }
}
