use super::*;


/// Basic reinforcement model parameters
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EnhanceLevel<T: Ord> {
    /// The level increased when the reinforcement is successful
    #[cfg_attr(feature = "serde", serde(default))]
    pub relative_rate: BTreeMap<i16, f64>,
    /// Jump to this level when strengthening fails
    #[cfg_attr(feature = "serde", serde(default))]
    pub absolute_rate: BTreeMap<u16, f64>,
    /// Weight of equipment broken
    #[cfg_attr(feature = "serde", serde(default))]
    pub broken_rate: f64,
    /// The amount of resources that need to be consumed for strengthening
    #[cfg_attr(feature = "serde", serde(default))]
    pub enhance_cost: BTreeMap<T, u128>,
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

    fn total_rate(&self) -> f64
    {


        self.broken_rate + self.relative_rate.values().sum::<f64>() + self.absolute_rate.values().sum::<f64>()
    }
}

