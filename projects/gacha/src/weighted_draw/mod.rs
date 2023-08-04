#![allow(clippy::wrong_self_convention)]

mod builder;
mod display;
mod sample;

use crate::Gacha;
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
}

impl<T> Gacha for WeightedList<T> {
    type Output = T;

    fn items(&self) -> usize {
        self.items.len()
    }

    fn contains(&self, other: &Self::Output) -> bool
    where
        Self::Output: PartialEq,
    {
        for item in &self.items {
            if item.key.eq(other) {
                return true;
            }
        }
        return false;
    }

    fn shuffle(&self, mut rng: impl Rng) -> Vec<&Self::Output> {
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

    fn random_next(&self, mut rng: impl Rng) -> Option<&Self::Output> {
        if self.items.is_empty() {
            panic!("No items in the list");
        }
        let total_weight = self.total_weight();
        let random_weight = rng.gen_range(0.0..total_weight);
        let index = self
            .items
            .binary_search_by(|elem| elem.accumulated.total_cmp(&random_weight)) //
            .unwrap_or_else(|i| i);
        Some(&self.items[index].key)
    }
    fn random_select(&self, count: usize, mut rng: impl Rng) -> Vec<&Self::Output> {
        match self.items.len() {
            0 => return vec![],
            1 => return vec![&self.items[0].key; count],
            _ => {
                let mut out = Vec::with_capacity(count);
                for _ in 0..count {
                    match self.random_next(&mut rng) {
                        Some(s) => out.push(s),
                        None => {}
                    }
                }
                out
            }
        }
    }
}
