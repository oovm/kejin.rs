

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum EnhanceState {
    /// The equipment is damaged (**absorbing state**)
    Broken,
    Level {
        level: u32,
        part: u32,
    },
}
