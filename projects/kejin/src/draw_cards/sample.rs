use rand::prelude::*;

fn weighted_shuffle<'a, T>(items: &'a [T], weights: &[usize]) -> Vec<&'a T> {
    let total_weight: usize = weights.iter().sum();
    let mut rng = thread_rng();
    let mut order: Vec<usize> = (0..items.len()).collect();
    order.sort_by_key(|&i| rng.gen_range(0..total_weight));

    order.iter().map(|&i| &items[i]).collect()
}

#[test]
fn main() {
    let items = vec!["item1", "item2", "item3" /* Add more items here */];
    let weights = vec![1, 2, 3 /* Add more weights here */];

    // 调用函数来获取加权随机样本
    let random_sampled_data = weighted_shuffle(&items, &weights);

    println!("{:?}", random_sampled_data);
}
