#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum EnhanceState {
    /// The equipment is damaged (**absorbing state**)
    Broken,
    Level {
        level: u32,
        part: u32,
    },
}

use nalgebra::{DMatrix, DVector};

// 定义一个函数，计算首次到达时间的分布
fn first_hitting_time_distribution(M: &DMatrix<f64>, final_state: usize, steps: usize) -> DVector<f64> {
    let n = M.nrows();
    let mut v0 = DVector::zeros(n);
    v0[0] = 1.0; // 初始向量为单位向量，从第一个状态开始

    // 计算 I + M
    let mut I_plus_M = M.clone();
    for i in 0..n {
        I_plus_M[(i, i)] += 1.0;
    }

    // 解方程组 I + M * τ = τ
    let mut tau = I_plus_M.lu().solve(&v0).expect("Solving linear equations failed.");

    // 将正无穷大的值转换为 0，表示无法从初始状态到达
    for i in 0..n {
        if tau[i].is_infinite() {
            tau[i] = 0.0;
        }
    }

    // 计算到达终态的分布
    let mut final_distribution = DVector::zeros(steps + 1);
    for step in 0..=steps {
        final_distribution[step] = tau[final_state].powf(step as f64);
    }

    final_distribution
}
#[test]
fn main() {
    // 示例矩阵 M
    let M = DMatrix::from_row_slice(3, 3, &[
        0.2, 0.3, 0.5,
        0.4, 0.1, 0.5,
        0.0, 0.0, 1.0,
    ]);

    let final_state = 2;
    let steps = 10;
    let distribution = first_hitting_time_distribution(&M, final_state, steps);
    println!("首次到达终态的分布：{:?}", distribution);
}