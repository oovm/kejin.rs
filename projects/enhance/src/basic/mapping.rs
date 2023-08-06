
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

    pub fn max_level(&self) -> usize {
        let mut max = 0;
        for (level, data) in &self.mapping {
            for i in data.relative_rate.keys() {
                let level = *level as usize + *i as usize;
                if level > max {
                    max = level;
                }
            }
            for i in data.absolute_rate.keys() {
                let level = *i as usize;
                if level > max {
                    max = level;
                }
            }
            if *level as usize > max {
                max = *level as usize;
            }
        }
        max
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

struct WolframMatrix<'a> {
    matrix: &'a DMatrix<f64>,
    rationalize: bool,
    breakable: bool,
}

impl<'a> Display for WolframMatrix<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("ℳ = ")?;
        if self.rationalize {
            f.write_str("Rationalize@")?;
        }
        for (i, row) in self.matrix.row_iter().enumerate() {
            if i > 0 {
                f.write_str("},\n    {")?;
            }
            else {
                f.write_str("{\n    {")?;
            }

            for (j, value) in row.iter().enumerate() {
                if j > 0 {
                    f.write_str(", ")?;
                }
                write!(f, "{}", value)?;
            }

        }
        f.write_str("}\n};\n")?;
        if self.breakable {
            f.write_str("𝒫 = DiscreteMarkovProcess[2, ℳ];")?
        }
        else {
            f.write_str("𝒫 = DiscreteMarkovProcess[1, ℳ];")?
        }
        writeln!(f, "Table[PDF[FirstPassageTimeDistribution[𝒫, 6], x], {{x, 0, 6*2}}]")
    }
}

impl EnhanceMatrix {
    pub fn as_wolfram(&self, rationalize: bool) -> String {
        let matrix = WolframMatrix {
            matrix: &self.matrix,
            rationalize,
            breakable: self.breakable,
        };
        matrix.to_string()
    }
}
