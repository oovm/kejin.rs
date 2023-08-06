
use super::*;

#[derive(Clone, Debug)]
pub struct EnhanceMap<T: Ord> {
    pub mapping: BTreeMap<u16, EnhanceLevel<T>>,
}

pub struct EnhanceMatrix {
    matrix: DMatrix<f64>,
    breakable: bool,
}

impl<T: Ord> Default for EnhanceMap<T> {
    fn default() -> Self {
        Self {
            mapping: BTreeMap::new(),
        }
    }
}

impl <T: Ord> EnhanceMap<T> {
    pub fn breakable(&self) -> bool {
        self.mapping.values().any(|level| level.broken_rate > 0.0)
    }

    pub fn as_matrix(&self) -> EnhanceMatrix {
        if self.breakable() {
            let mut matrix = DMatrix::from_element(self.mapping.len(), self.mapping.len(), 0.0);
            for (i, level) in self.mapping.values().enumerate() {
                for (j, rate) in level.relative_rate.iter() {
                    matrix[(i, i + *j as usize)] = *rate;
                }
                for (j, rate) in level.absolute_rate.iter() {
                    matrix[(i, *j as usize)] = *rate;
                }
                matrix[(i, i)] = level.broken_rate;
            }
            EnhanceMatrix {
                matrix,
                breakable: true,
            }
        } else {
            let mut matrix = DMatrix::from_element(self.mapping.len(), self.mapping.len(), 0.0);
            for (i, level) in self.mapping.values().enumerate() {
                for (j, rate) in level.relative_rate.iter() {
                    matrix[(i, i + *j as usize)] = *rate;
                }
                for (j, rate) in level.absolute_rate.iter() {
                    matrix[(i, *j as usize)] = *rate;
                }
            }
            EnhanceMatrix {
                matrix,
                breakable: false,
            }
        }
    }
}