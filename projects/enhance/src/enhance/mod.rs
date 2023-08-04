use std::collections::BTreeSet;

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EnhanceLevel<T> {
    pub success_rate: BTreeSet<i32, f32>,
    pub broken_rate: f32,
    pub enhance_cost: BTreeSet<T, f32>,
    pub protect_cost: BTreeSet<T, f32>,
}

impl<T> EnhanceLevel<T> {
    fn total_rate(&self) -> f32 {
        self.success_rate.values().sum::<f32>() + self.broken_rate
    }
}
