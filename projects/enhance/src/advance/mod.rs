use crate::EnhanceLevel;
use std::collections::BTreeMap;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EnhanceModifier<T> {
    /// Situation when no additional material is used
    pub basic: EnhanceLevel<T>,
    /// The level increased when the reinforcement is successful
    #[cfg_attr(feature = "serde", serde(default))]
    pub relative_rate: BTreeMap<i16, f64>,
    /// Jump to this level when strengthening fails
    #[cfg_attr(feature = "serde", serde(default))]
    pub absolute_rate: BTreeMap<u16, f64>,
    /// Weight of equipment broken
    #[cfg_attr(feature = "serde", serde(default))]
    pub broken_rate: f64,
    /// The number of resources that need to be consumed in order to prevent failure penalties
    #[cfg_attr(feature = "serde", serde(default))]
    pub additional: BTreeMap<T, f64>,
}
