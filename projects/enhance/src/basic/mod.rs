pub mod level;
pub mod state;
pub mod display;
pub mod mapping;
#[cfg(feature = "serde")]
pub mod ser_der;

use crate::EnhanceLevel;
use std::{
    collections::{BTreeMap, BTreeSet},
    ops::Add,
};
use std::fmt::{Display, Formatter, Write};
use nalgebra::DMatrix;

