use std::fmt::Display;

use crate::day_1::{combination::Combination, direction::Direction};

pub struct SafeCode {
    current: i16,
    zero_seen: i16,
    max: i16
}

impl SafeCode {
    pub fn new() -> Self {
        SafeCode {
            current: 50,
            zero_seen: 0,
            max: 99
        }
    }

    pub fn rotate(&mut self, combination: Combination) {
        let rotation = match combination.direction {
            Direction::Left => -combination.amount,
            Direction::Right => combination.amount,
        };
        let mut amount = combination.amount;
        if amount >= 100 {
            let passes = amount / (self.max + 1);
            amount = amount % (self.max + 1);
            self.zero_seen += passes;
        }
        if combination.direction == Direction::Left {
            if self.current - amount == 0 {
                self.zero_seen += 1;
            }
            if self.current - amount < 0 && self.current != 0{
                self.zero_seen += 1;
            }

        }
        if combination.direction == Direction::Right {
           if self.current + amount > self.max {
                self.zero_seen += 1;
            }
        }
        self.current = (self.current + rotation).rem_euclid(self.max + 1);

        // Part 1 solution
        // if self.current == 0  {
        //     println!("Hit zero!");
        //     self.zero_seen += 1;
        // }
    }
}

impl Display for SafeCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.current, self.zero_seen)
    }
}