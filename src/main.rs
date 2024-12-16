use std::vec;

use sha2::digest::generic_array::iter;

struct FilterCondition {
    filter: Vec<i32>,
}

trait FilterTrait {
    fn is_match(&self, val: i32) -> bool;
}

impl FilterTrait for FilterCondition {
    fn is_match(&self, val: i32) -> bool {
        val > 5
    }
}

fn custom_filter(collection: &Vec<i32>, obj: FilterCondition) -> Vec<i32> {
    collection
        .iter()
        .filter(|&&x| obj.is_match(x))
        .cloned()
        .collect()
}

fn main() {
    let vector = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let condition = FilterCondition { filter: vector.clone() };
    let new = custom_filter(&vector, condition);
    println!("result : {:?}", new);
}
