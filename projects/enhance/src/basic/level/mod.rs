use super::*;


/// Basic reinforcement model parameters
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EnhanceLevel<T, R> {
    /// The level increased when the reinforcement is successful
    #[cfg_attr(feature = "serde", serde(default))]
    pub relative_rate: BTreeMap<i16, R>,
    /// Jump to this level when strengthening fails
    #[cfg_attr(feature = "serde", serde(default))]
    pub absolute_rate: BTreeMap<u16, R>,
    /// Weight of equipment broken
    #[cfg_attr(feature = "serde", serde(default))]
    pub broken_rate: R,
    /// The amount of resources that need to be consumed for strengthening
    #[cfg_attr(feature = "serde", serde(default))]
    pub enhance_cost: BTreeMap<T, u128>,
}




impl<T, R> EnhanceLevel<T, R> {
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
    pub fn simple(success: R, failure: R, change: i16) -> Self where R:Default {
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

    fn total_rate(&self) -> R
    where
        R: Clone + Add<R, Output = R>,
    {
        let mut out = self.broken_rate.clone();
        for weight in self.relative_rate.values() {
            out = out.add(weight.clone());
        }
        for weight in self.absolute_rate.values() {
            out = out.add(weight.clone());
        }
        out
    }
}

