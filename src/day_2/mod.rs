use std::fs;

use crate::day_2::{invalid_id_strategy::{InvalidIdStrategy, Part1Strategy, Part2Strategy}, range::Range};

mod product_id;
mod range;
mod invalid_id_strategy;

pub fn day_2() {
    let contents = fs::read_to_string("./src/day_2/input.txt").expect("Could not read file");
    let total = contents.split(",")
    .map(|x| { 
        let range = Range::<Part2Strategy>::from(x);
        let invalid_ids = range.strategy.find_invalid_ids(range);
        let mut range = Range::<Part2Strategy>::from(x);
        range.invalid_ids = invalid_ids; 
        range })
    .fold(0, |total, range| total + range.invalid_ids.iter().map(|id| id.id).sum::<u128>());
    println!("Total invalid IDs sum: {}", total);
}