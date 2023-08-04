use rand::Rng;

pub trait Gacha {
    type Output;
    /// Count the number of elements
    fn items(&self) -> usize;
    fn contains(&self, other: &Self::Output) -> bool
    where
        Self::Output: PartialEq;
    /// Rearrange all elements according to weight, the higher the weight, the higher the front.
    fn shuffle(&self, rng: impl Rng) -> Vec<&Self::Output>;
    /// Randomly select an element based on weight, the higher the weight, the easier it is to be selected.
    fn random_next(&self, rng: impl Rng) -> Option<&Self::Output>;
    /// Select n elements according to the weight, the higher the weight, the easier it is to be selected
    fn random_select(&self, count: usize, mut rng: impl Rng) -> Vec<&Self::Output> {
        let mut out = Vec::with_capacity(count);
        for _ in 0..count {
            match self.random_next(&mut rng) {
                Some(s) => out.push(s),
                None => {}
            }
        }
        out
    }
    /// Select n unique elements according to the weight, the higher the weight, the higher the front
    fn random_sample(&self, count: usize, mut rng: impl Rng) -> Vec<&Self::Output> {
        let mut shuffle = self.shuffle(&mut rng);
        shuffle.truncate(count);
        shuffle
    }
}
