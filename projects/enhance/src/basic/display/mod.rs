use super::*;


pub struct TeXTable {

}
pub struct TeXMatrix {

}


pub struct WolframFormat {
    matrix: DMatrix<f64>,
    rationalize: bool,
    destroyable: bool,
}

impl Display for WolframFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("ℳ = ")?;
        if self.rationalize {
            f.write_str("Rationalize@")?;
        }
        f.write_char('{')?;
        for row in self.matrix.row_iter() {
            f.write_char('{')?;
            for (i, &x) in row.iter().enumerate() {
                if i != 0 {
                    f.write_char(',')?;
                }
                f.write_fmt(format_args!("{}", x))?;
            }
            f.write_str("},")?;
        }
        f.write_str("};")?;
        f.write_str("ℙ = DiscreteMarkovProcess[1, ℳ];")?;
        f.write_str("Table[PDF[FirstPassageTimeDistribution[ℙ, 6], x], {x, 0, 6*2}]")?;
        Ok(())

    }
}
