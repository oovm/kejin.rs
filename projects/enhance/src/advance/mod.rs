use crate::EnhanceLevel;
use std::collections::BTreeMap;

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EnhanceModifier<T, R> {
    /// Situation when no additional material is used
    pub basic: EnhanceLevel<T, R>,
    /// The level increased when the reinforcement is successful
    #[cfg_attr(feature = "serde", serde(default))]
    pub relative_rate: BTreeMap<i16, R>,
    /// Jump to this level when strengthening fails
    #[cfg_attr(feature = "serde", serde(default))]
    pub absolute_rate: BTreeMap<u16, R>,
    /// Weight of equipment broken
    #[cfg_attr(feature = "serde", serde(default))]
    pub broken_rate: R,
    /// The number of resources that need to be consumed in order to prevent failure penalties
    #[cfg_attr(feature = "serde", serde(default))]
    pub additional: BTreeMap<T, u128>,
}
