mod combination;
mod safe_code;
mod direction;

use std::fs;
use combination::Combination;
use safe_code::SafeCode;


pub fn day_1() {
    let mut safe = SafeCode::new();
    let contents = fs::read_to_string("./src/day_1/input.txt").expect("Could not read file");
    for line in contents.lines() {
        let combination = Combination::parse_from_str(line);
        safe.rotate(combination);
    }
    println!("Result: {}", safe);
}
