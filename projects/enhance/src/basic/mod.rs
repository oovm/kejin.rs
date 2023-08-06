pub mod level;
pub mod state;
pub mod display;
pub mod mapping;

use crate::EnhanceLevel;
use std::{
    collections::{BTreeMap, BTreeSet},
    ops::Add,
};
use std::fmt::{Display, Formatter, Write};
use nalgebra::DMatrix;

pub struct EnhanceCost<T> {
    items: BTreeMap<T, u128>,
}
