use super::*;

impl<T: PartialEq> PartialEq<Self> for WeightedElement<T> {
    fn eq(&self, other: &Self) -> bool {
        self.key.eq(&other.key) && self.weight.eq(&other.weight) && self.accumulated.eq(&other.accumulated)
    }
}

impl<T: Eq> Eq for WeightedElement<T> {}
