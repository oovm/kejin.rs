use super::*;

use rand::prelude::*;

struct WeightedSampler<'a, T> {
    items: &'a [T],
    weights: &'a [f32],
    nodes: Vec<[f32; 3]>,
}

impl<'a, T> WeightedSampler<'a, T> {
    fn new(items: &'a [T], weights: &'a [f32]) -> Self {
        if items.len() != weights.len() {
            panic!("Unequal lengths");
        }

        let n = items.len();
        let nodes: Vec<[f32; 3]> = vec![[0.0; 3]; n];

        WeightedSampler {
            items,
            weights,
            nodes,
        }
    }

    fn left_index(&self, i: usize) -> usize {
        2 * i + 1
    }

    fn right_index(&self, i: usize) -> usize {
        2 * i + 2
    }

    fn total_weight(&mut self, i: usize) -> f32 {
        if i >= self.items.len() {
            return 0.0;
        }
        let this_weight = self.weights[i];
        if this_weight <= 0.0 {
            panic!("weight can't be zero or negative");
        }
        let left_weight = self.total_weight(self.left_index(i));
        let right_weight = self.total_weight(self.right_index(i));
        self.nodes[i] = [this_weight, left_weight, right_weight];
        this_weight + left_weight + right_weight
    }

    fn sample(&mut self, i: usize) -> usize {
        let [this_w, left_w, right_w] = self.nodes[i];
        let total = this_w + left_w + right_w;
        let r = total * random::<f32>();
        if r < this_w {
            self.nodes[i][0] = 0.0;
            i
        } else if r < this_w + left_w {
            let chosen = self.sample(self.left_index(i));
            self.nodes[i][1] -= self.nodes[chosen][0];
            chosen
        } else {
            let chosen = self.sample(self.right_index(i));
            self.nodes[i][2] -= self.nodes[chosen][0];
            chosen
        }
    }

    fn weighted_shuffle(&mut self) -> Vec<&T> {
        self.total_weight(0);
        (0..self.items.len() - 1).map(|_| self.items[self.sample(0)]).collect()
    }
}

#[test]
fn main() {
    let items = vec!["item1", "item2", "item3" /* Add more items here */];
    let weights = vec![0.5, 0.3, 0.7 /* Add more weights here */];

    let mut sampler = WeightedSampler::new(&items, &weights);

    // 调用方法来获取5个加权随机样本
    let random_sampled_data = sampler.weighted_shuffle();

    println!("{:?}", random_sampled_data);
}