
use super::*;

#[derive(Clone, Debug)]
pub struct EnhanceMap<T: Ord> {
    pub mapping: BTreeMap<u16, EnhanceLevel<T>>,
}

#[derive(Clone, Debug)]
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

    pub fn max_level(&self) -> u16 {
        *self.mapping.keys().max().unwrap_or(&0)
    }

    pub fn as_matrix(&self) -> EnhanceMatrix {
        if self.breakable() {
            unimplemented!()
        } else {
            let rank = self.max_level() as usize +1;
            let mut matrix = DMatrix::from_element(rank, rank, 0.0);
            for (level, enhance) in self.mapping.iter() {
                let total_rate = enhance.total_rate();
                for (change, rate) in enhance.relative_rate.iter() {
                    let change = *change as usize;
                    matrix[(*level as usize, change)] = rate / total_rate;
                }
                for (change, rate) in enhance.absolute_rate.iter() {
                    let change = *change as usize;
                    matrix[(*level as usize, change)] = rate / total_rate;
                }
            }
            EnhanceMatrix {
                matrix,
                breakable: false,
            }
        }
    }
}

impl EnhanceMatrix {
    pub fn as_wolfram(&self, rationalize: bool) -> String {
        let mut out = String::new();
        out.push_str("ℳ = ");
        if rationalize {
            out.push_str("Rationalize@");
        }
        out.push_str("{");
        for (i, row) in self.matrix.row_iter().enumerate() {
            if i > 0 {
                out.push_str(", ");
            }
            out.push_str("{");
            for (j, value) in row.iter().enumerate() {
                if j > 0 {
                    out.push_str(", ");
                }
                out.push_str(&value.to_string());
            }
            out.push_str("}");
        }
        out.push_str("};");
        out.push_str("ℙ = DiscreteMarkovProcess[1, ℳ];");
        out.push_str("Table[PDF[FirstPassageTimeDistribution[ℙ, 6], x], {x, 0, 6*2}]");
        out
    }
}
