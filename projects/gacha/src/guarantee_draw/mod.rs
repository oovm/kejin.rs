use crate::Gacha;
use rand::Rng;
use std::{cell::RefCell, collections::BTreeSet, ops::AddAssign};

/// Hard guarantee card draw mechanism, if the number of guarantees is reached and the limited card is still not obtained, the card will be issued forcibly
pub struct GuaranteeDraw<G: Gacha> {
    pub current: RefCell<usize>,
    pub max: usize,
    pub normal: G,
    pub guarantee: G,
}

pub struct GuaranteeCondition<G: Gacha> {
    pub max: usize,
    pub gacha: G,
    pub reset: BTreeSet<G::Output>,
}

impl<G: Gacha> GuaranteeDraw<G> {
    pub fn new(model: G, guarantee: G, max: usize) -> Self {
        Self { current: RefCell::new(0), max, normal: model, guarantee }
    }
    pub fn with_current(mut self, current: usize) -> Self {
        self.current = RefCell::new(current);
        self
    }
}

impl<G> Gacha for GuaranteeDraw<G>
where
    G: Gacha,
    G::Output: Eq,
{
    type Output = G::Output;

    fn items(&self) -> usize {
        self.normal.items()
    }

    fn contains(&self, other: &Self::Output) -> bool {
        todo!()
    }

    fn shuffle(&self, rng: impl Rng) -> Vec<&Self::Output> {
        self.normal.shuffle(rng)
    }

    fn random_next(&self, rng: impl Rng) -> Option<&Self::Output> {
        let current = *self.current.borrow();
        if current < self.max {
            self.current.replace(current + 1);
            let out = self.normal.random_next(rng)?;
            if self.guarantee.contains(&out) {
                self.current.replace(0);
            }
            Some(out)
        }
        else {
            self.current.replace(0);
            self.guarantee.random_next(rng)
        }
    }
}
