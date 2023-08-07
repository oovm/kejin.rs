
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
        self.mapping.iter().map(|(level, data)|data.max_level(*level)).max().unwrap_or(0)
    }

    pub fn as_matrix(&self) -> EnhanceMatrix {
        if self.breakable() {
            unimplemented!()
        } else {
            let max = self.max_level() as usize;
            let mut matrix = DMatrix::from_element(max +1, max +1, 0.0);
            for (base, data) in &self.mapping {
               for (level, rate) in  data.as_absolute(*base).absolute_rate {
                     matrix[(*base as usize, level as usize)] = rate;
               }
            }
            matrix[(max, max)] = 1.0;
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
            f.write_str("𝒫 = DiscreteMarkovProcess[2, ℳ];\n")?;
            writeln!(f, "Table[PDF[FirstPassageTimeDistribution[𝒫, 6], x], {{x, 0, 6*2}}]")
        }
        else {
            f.write_str("𝒫 = DiscreteMarkovProcess[1, ℳ];\n")?;
            let rows = self.matrix.nrows();
            f.write_str(r#"

steps = Range[1,18];
selected = Range[1,5];
legends = "+"<>ToString[#]&/@selected;
𝒫 = DiscreteMarkovProcess[1,ℳ];

cdf=Table[{j,CDF[FirstPassageTimeDistribution[𝒫,i],j]},{i,selected+1},{j,steps}];
mean=Table[Mean@FirstPassageTimeDistribution[𝒫,i],{i,5}];
m1=Table[Quantile[FirstPassageTimeDistribution[𝒫,i],0.05],{i,5}];
m2=Table[Quantile[FirstPassageTimeDistribution[𝒫,i],0.25],{i,5}];
m3=Table[Quantile[FirstPassageTimeDistribution[𝒫,i],0.50],{i,5}];
m4=Table[Quantile[FirstPassageTimeDistribution[𝒫,i],0.75],{i,5}];
m5=Table[Quantile[FirstPassageTimeDistribution[𝒫,i],0.95],{i,5}];

plotPDF[]:=Block[
    {pdf},
    pdf=Table[{j,PDF[FirstPassageTimeDistribution[𝒫,i],j]},{i,selected+1},{j,steps}];
    ListLinePlot[pdf,PlotLegends->legends,AxesLabel->{"强化次数","达成概率"},PlotLabel->"首次达成概率", PlotRange->Full, Mesh->Full,PlotTheme->"FullAxesGrid",PlotStyle->24]
];
plotPDF[]
cp=ListLinePlot[cdf, PlotRange->Full,Mesh->Full,PlotLegends->legends,AxesLabel->{"强化次数","达成概率"},PlotStyle->24,PlotLabel->"累计达成概率",PlotTheme->"Scientific"]
mp=ListLinePlot[{mean,m1,m2,m3,m4,m5},AxesLabel->{"强化等级","强化次数"},PlotLegends->{"平均达成次数","5%达成次数","25%达成次数","中位达成次数","75%达成次数","95%达成次数"},PlotStyle->114, PlotRange->Full,Mesh->Full,PlotTheme->{"Default", "Grid"}]

            "#)

        }
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
