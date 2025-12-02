use crate::day_1::direction::Direction;


pub struct Combination {
    pub direction: Direction,
    pub amount: i16,
}

impl Combination {
    pub fn parse_from_str(s: &str) -> Self {
        let (dir_char, amount_str) = s.split_at(1);
        let direction = match dir_char {
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("Invalid direction character"),
        };
        let amount = amount_str.parse::<i16>().expect("Invalid amount");
        Combination { direction, amount }
    }
}