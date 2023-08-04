use crate::Gacha;
use rand::Rng;
use std::{cell::RefCell, collections::BTreeSet, num::NonZeroUsize, ops::AddAssign};

/// Hard guarantee card draw mechanism, if the number of guarantees is reached and the limited card is still not obtained, the card will be issued forcibly
pub struct GuaranteeDraw<G: Gacha> {
    pub current: RefCell<usize>,
    pub normal: G,
    pub guarantee: Vec<GuaranteeCondition<G>>,
}

pub struct GuaranteeCondition<G: Gacha> {
    pub items: G,
    pub max: Option<NonZeroUsize>,
    pub reset: fn(G::Output) -> bool,
}

impl<G: Gacha> GuaranteeDraw<G> {
    pub fn new(cards: G) -> Self {
        Self { current: RefCell::new(0), normal: cards, guarantee: vec![] }
    }
    pub fn with_current(mut self, current: usize) -> Self {
        self.current = RefCell::new(current);
        self
    }
    pub fn with_guarantee(mut self, guarantee: GuaranteeCondition<G>) -> Self {
        self.guarantee.push(guarantee);
        self
    }
}

impl<G: Gacha> GuaranteeCondition<G> {
    pub fn new(cards: G) -> Self {
        Self { items: cards, max: None, reset: |_| false }
    }
    pub fn with_max(mut self, max: usize) -> Self {
        self.max = NonZeroUsize::new(max);
        self
    }
    pub fn with_reset(mut self, reset: fn(G::Output) -> bool) -> Self {
        self.reset = reset;
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
