use std::fs;

use crate::day_2::range::Range;

mod product_id;
mod range;

pub fn day_2() {
    let contents = fs::read_to_string("./src/day_2/input.txt").expect("Could not read file");
    let total = contents.split(",")
    .map(|x| { 
        let mut range = Range::from(x); 
        range.find_invalid_ids(); 
        range })
    .fold(0, |total, range| total + range.invalid_ids.iter().map(|id| id.id).sum::<u128>());
    println!("Total invalid IDs sum: {}", total);
}