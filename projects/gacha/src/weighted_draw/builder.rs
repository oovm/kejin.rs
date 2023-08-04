use super::*;

impl<T> WeightedList<T> {
    /// Create a new empty weighted list with given capacity
    pub fn new(capacity: usize) -> Self {
        Self { items: Vec::with_capacity(capacity), total: 0.0 }
    }
}

impl<T> AddAssign<(T, f64)> for WeightedList<T> {
    fn add_assign(&mut self, rhs: (T, f64)) {
        if rhs.1 >= 0.0 {
            self.total += rhs.1;
            self.items.push(WeightedElement { key: rhs.0, weight: rhs.1, accumulated: self.total });
        }
        else {
            // nan, negative, zero, inf
            log::warn!("element ignored, {} is not positive", rhs.1);
        }
    }
}

impl<T> AddAssign<(T, usize)> for WeightedList<T> {
    fn add_assign(&mut self, rhs: (T, usize)) {
        let weight = rhs.1 as f64;
        self.total += weight;
        self.items.push(WeightedElement { key: rhs.0, weight, accumulated: self.total });
    }
}

impl<T> FromIterator<(T, f64)> for WeightedList<T> {
    fn from_iter<I: IntoIterator<Item = (T, f64)>>(iter: I) -> Self {
        let sequence = iter.into_iter();
        let mut out = WeightedList::new(sequence.size_hint().0);
        for (key, weight) in sequence {
            out += (key, weight);
        }
        out
    }
}

impl<T> FromIterator<(T, usize)> for WeightedList<T> {
    fn from_iter<I: IntoIterator<Item = (T, usize)>>(iter: I) -> Self {
        let sequence = iter.into_iter();
        let mut out = WeightedList::new(sequence.size_hint().0);
        for (key, weight) in sequence {
            out += (key, weight);
        }
        out
    }
}

impl<'i, T> IntoIterator for &'i WeightedList<T> {
    type Item = &'i WeightedElement<T>;
    type IntoIter = Iter<'i, WeightedElement<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.iter()
    }
}
