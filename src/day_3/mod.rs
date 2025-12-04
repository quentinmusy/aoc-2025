use std::fs;

mod bank;


pub fn day_3() {
    // Part 1
    let contents = fs::read_to_string("./src/day_3/input.txt").expect("Could not read file");
    let total = contents.lines().map(|line| {
        let bank = bank::Bank::from(line);
        bank.best_two_batteries()
    }).fold(0, |total: u32, best| total + best as u32);
    println!("Total best two batteries sum: {}", total);

    // Part 2

    let contents = fs::read_to_string("./src/day_3/input.txt").expect("Could not read file");
    let total = contents.lines().map(|line| {
        let bank = bank::Bank::from(line);
        let best =bank.best_n_batteries(12);
        println!("Best for line {}: {}", line, best);
        best
    }).fold(0, |total: u64, best| total + best as u64);
    println!("Total best two batteries sum: {}", total);

}