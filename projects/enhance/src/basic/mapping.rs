
use super::*;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EnhanceMap<T: Ord + Default> {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub mapping: BTreeMap<u16, EnhanceLevel<T>>,
}