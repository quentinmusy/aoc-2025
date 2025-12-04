pub struct Bank {
    batteries: Vec<u8>
}

impl Bank {
    pub fn new() -> Self {
        Bank {
            batteries: Vec::new()
        }
    }

    pub fn best_two_batteries(&self) -> u64 {
        let mut max: u64 = 0;
        for i in 0..self.batteries.len() {
            for j in i+1..self.batteries.len() {
                let sum = self.batteries[i] as u64 * 10 + self.batteries[j] as u64;
                if sum > max {
                    max = sum
                }
            }
        }
        max
    }


    pub fn best_n_batteries(&self, n: usize) -> u64 {
        let mut jolt_str = String::new();
        let mut digit_index = 0;


        for i in 0..n {
            let mut largest_digit: i8 = -1;
            let remaining = n - i - 1;
            let mut best_idx = 0;
            for (idx, battery) in self.batteries[digit_index..self.batteries.len()-remaining].iter().enumerate() {
                if *battery as i8 > largest_digit {
                    largest_digit = *battery as i8;
                    best_idx = idx + 1;
                }
            }
            jolt_str += &largest_digit.to_string();
            digit_index += best_idx;
        }
        jolt_str.parse::<u64>().unwrap()
       
    }
}

impl From<&str> for Bank {
    fn from(s: &str) -> Self {
        let batteries = s.chars().map(|c| c.to_digit(10).unwrap() as u8).collect();
        Bank {
            batteries
        }
    }
}


#[cfg(test)]
mod tests_part_1 {
    use super::*;

    macro_rules! best_two_batteries_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (bank_str, expected) = $value;
                let start = Bank::from(bank_str);
                assert_eq!(start.best_two_batteries(), expected);
            }
        )*
        }
    }

    best_two_batteries_tests!(
        test_one: ("1234567", 67),
        test_two: ("987654321", 98),
        test_three: ("11111", 11),
        test_four: ("54321", 54),
        disparate: ("914321118", 98),
        example_1: ("987654321111111", 98),
        example_2: ("811111111111119", 89),
        example_3: ("234234234234278", 78),
        example_4: ("818181911112111", 92),
    );

}

#[cfg(test)]
mod tests_part_2 {
    use super::*;

    macro_rules! best_12_batteries_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (bank_str, expected) = $value;
                let start = Bank::from(bank_str);
                assert_eq!(start.best_n_batteries(12), expected);
            }
        )*
        }
    }

    best_12_batteries_tests!(
        example_1: ("987654321111111", 987654321111),
        example_2: ("811111111111119", 811111111119),
        example_3: ("234234234234278", 434234234278),
        

        example_4: ("818181911112111", 888911112111),
    );
}