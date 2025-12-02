use crate::day_1::{combination::Combination, direction::Direction};

pub struct SafeCode {
    current: i16,
    time_at_zero: i16,
    max: i16
}

impl SafeCode {
    pub fn new() -> Self {
        SafeCode {
            current: 50,
            time_at_zero: 0,
            max: 99
        }
    }

    pub fn rotate(&mut self, combination: Combination) {
        let rotation = match combination.direction {
            Direction::Left => -combination.amount,
            Direction::Right => combination.amount,
        };
        println!("{} {:?}", combination.amount, combination.direction);
        let mut amount = combination.amount;
        if amount >= 100 {
            let passes = amount / (self.max + 1);
            amount = amount % (self.max + 1);
            self.time_at_zero += passes;
            println!("Passed zero {} times ", passes);
        }
        if combination.direction == Direction::Left {
            if self.current - amount == 0 {
                self.time_at_zero += 1;
                println!("passed zero left!");
            }
            if self.current - amount < 0 && self.current != 0{
                self.time_at_zero += 1;
                println!("passed zero left!");
            }

        }
        if combination.direction == Direction::Right {
           if self.current + amount > self.max {
                self.time_at_zero += 1;
                println!("passed zero right!");
            }
        }
        self.current = (self.current + rotation).rem_euclid(self.max + 1);

        // if self.current == 0  && !already_counted {
        //     println!("Hit zero!");
        //     self.time_at_zero += 1;
        // }
    }
}
