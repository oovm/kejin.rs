use std::mem::take;
use super::*;


/// Basic reinforcement model parameters
#[derive(Clone, Debug)]
pub struct EnhanceLevel<T: Ord> {
    /// The level increased when the reinforcement is successful
    pub relative_rate: BTreeMap<i16, f64>,
    /// Jump to this level when strengthening fails
    pub absolute_rate: BTreeMap<u16, f64>,
    /// Weight of equipment broken
    pub broken_rate: f64,
    /// The amount of resources that need to be consumed for strengthening
    pub enhance_cost: BTreeMap<T, u128>,
}
#[derive(Clone, Debug)]
pub struct EnhanceTransition {
    /// Jump to this level when strengthening fails
    pub absolute_rate: BTreeMap<u16, f64>,
    /// Weight of equipment broken
    pub broken_rate: f64,
}

impl<T: Ord> Default for EnhanceLevel<T> {
    fn default() -> Self {
        Self {
            relative_rate: BTreeMap::new(),
            absolute_rate: BTreeMap::new(),
            broken_rate: 0.0,
            enhance_cost: BTreeMap::new(),
        }
    }
}


impl<T: Ord> EnhanceLevel<T> {
    /// Create a simple reinforcement model
    ///
    /// # Arguments
    ///
    /// * `success`: The success rate of the first level of reinforcement
    /// * `failure`: The failure rate of the first level of reinforcement
    /// * `change`: Change level when reinforcement fails
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    pub fn simple(success: f64, failure: f64, change: i16) -> Self{
        let mut relative_rate = BTreeMap::new();
        relative_rate.insert(1, success);
        relative_rate.insert(change, failure);
        Self {
            relative_rate,
            absolute_rate: BTreeMap::new(),
            broken_rate: Default::default(),
            enhance_cost: BTreeMap::new(),
        }
    }
    pub fn total_rate(&self) -> f64
    {
        self.broken_rate + self.relative_rate.values().sum::<f64>() + self.absolute_rate.values().sum::<f64>()
    }
    pub fn as_absolute(&self, base: u16) -> EnhanceTransition {
        let total = self.total_rate();
        let mut absolute_rate = BTreeMap::default();
        for (level, data) in &self.absolute_rate {
            absolute_rate.insert(*level, *data / total);
        }
        for (level, data) in &self.relative_rate {
            let level = if level.is_negative() {
             base.saturating_sub(level.abs() as u16)

            }
            else {
              base.saturating_add(*level as u16)

            };
            absolute_rate.insert(level, *data / total);
        }
        EnhanceTransition {
            absolute_rate,
            broken_rate: self.broken_rate / total,
        }
    }
    /// Compute the highest grade from the base grade
    pub fn max_level(&self, base: u16) -> u16 {
        let mut max = base;
        for &i in self.relative_rate.keys() {
            if i.is_negative() {
                continue;
            }
            let level = i as u16 + base;
            if level > max {
                max = level;
            }
        }
        for &level in self.absolute_rate.keys() {
            if level > max {
                max = level;
            }
        }
        max
    }
}

