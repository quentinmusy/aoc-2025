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

    fn recenter(&mut self) {
        if self.current < 0 {
            self.current = self.max + 1 + self.current;
        }
        if self.current > self.max {
            self.current = self.current % (self.max + 1);
        }
    }

    pub fn rotate(&mut self, combination: Combination) {
        // Part 2 solution

        let mut amount = combination.amount;
        // Number of full passes, for large numbers
        if amount >= 100 {
            let passes = amount / (self.max + 1);
            amount = amount % (self.max + 1);
            self.zero_seen += passes;
        }

        if combination.direction == Direction::Left {
            let diff = self.current - amount;
            // We don't want to count twice if we were already at zero
            if diff <= 0 && self.current != 0 {
                self.zero_seen += 1;
            }
            self.current = diff;

        }
        if combination.direction == Direction::Right {
           if self.current + amount > self.max {
                self.zero_seen += 1;
            }
            self.current += amount;
        }
        self.recenter();

        // Part 1 solution
        // let rotation = match combination.direction {
        //     Direction::Left => -combination.amount,
        //     Direction::Right => combination.amount,
        // };

        //self.current = (self.current + rotation).rem_euclid(self.max + 1);

        // if self.current == 0  {
        //     self.zero_seen += 1;
        // }
    }
}

impl Display for SafeCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.current, self.zero_seen)
    }
}