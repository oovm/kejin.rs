pub mod level;
pub mod state;

use std::{
    collections::{BTreeMap, BTreeSet},
    ops::Add,
};

pub struct EnhanceCost<T> {
    items: BTreeMap<T, u128>,
}
