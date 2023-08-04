use rand::prelude::StdRng;
use rand::SeedableRng;
use gacha::WeightedList;

#[test]
fn ready() {
    println!("it works!")
}


#[test]
fn test_weighted_list_shuffle() {
    let items = vec![("item1", 1), ("item2", 10), ("item3", 100), ("item4", 1000), ("item5", 10000)];
    let weighted_list = WeightedList::from_iter(items.into_iter().map(|(key, weight)| (key, weight as f64)));

    let mut rng = StdRng::from_entropy();
    let shuffled_items = weighted_list.shuffle(&mut rng);
    println!("{:?}", shuffled_items);
}