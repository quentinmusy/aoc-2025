use std::collections::HashSet;

use crate::day_2::{product_id::ProductId, range::Range};

pub trait InvalidIdStrategy: Sized {
    fn find_invalid_ids(self, range: Range<Self>) -> Vec<ProductId>;
}

#[derive(Default, Clone, Copy)]
pub struct Part1Strategy;

impl InvalidIdStrategy for Part1Strategy {
    /**
     * We don't want to check every ID in the range, as ranges can be quite big,
     * so we can try to find possible invalid IDs based on their structure.
     * We know that invalid IDs have the same digits in both halves.
     * So we need to look at first half of first ID and last half of last ID
     * to find possible candidates.
     */
    fn find_invalid_ids(self, range: Range<Self>) -> Vec<ProductId> {
        let (start_first_half, _) = range.start.get_halves(false);
        let (start_second_half, _) = range.end.get_halves(true);
        let mut invalid_ids = vec![];
        // Compute all possible "doubles" in the range
        // And check if they are within the range
        let start = start_first_half.parse::<u128>().unwrap_or(0);
        let end = start_second_half.parse::<u128>().unwrap_or(0);
        for half in std::cmp::min(start, end)..=std::cmp::max(start, end) {
            let half_str = half.to_string();
            let possible_id_str = format!("{}{}", half_str, half_str);
            if let Ok(possible_id) = possible_id_str.parse::<u128>() {
                if possible_id >= range.start.id && possible_id <= range.end.id {
                    invalid_ids.push(ProductId::new(possible_id));
                }
            }
        }
        invalid_ids
    }
}

#[derive(Default, Clone, Copy)]
pub struct Part2Strategy;

impl InvalidIdStrategy for Part2Strategy {
    /**
     * Turns out, we do need to check every ID in the range for part 2,
     * as computing the possible invalid IDs takes even more time, or I have not found
     * the correct way to do it.
     * Instead we look for all possible size between 1 and half the size of the ID,
     * and check if all parts are the same.
     * For each id.
     */
    fn find_invalid_ids(self, range: Range<Self>) -> Vec<ProductId> {
        let mut invalid_ids: HashSet<ProductId> = HashSet::new();
        for i in range.start.id..=range.end.id {
            let product_id = ProductId::new(i);
            for j in 1..=(product_id.id.to_string().len() / 2) {
                let parts = product_id.cut_in_sizes(j);
                if parts.len() < 2 {
                    continue;
                }
                let all_equal = parts.iter().all(|part| part == &parts[0]);
                if all_equal {
                    invalid_ids.insert(product_id);
                    break;
                }
            }
        }

        let mut ids = invalid_ids.into_iter().collect::<Vec<ProductId>>();
        ids.sort();
        ids
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! part_2_strategy_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (start_number, end_number, expected) = $value;
                let start = ProductId::new(start_number);
                let end = ProductId::new(end_number);
                let mut range = Range::new(start, end, Part2Strategy::default());
                let invalid_ids = range.strategy.find_invalid_ids(range);
                range = Range::new(start, end, Part2Strategy::default());
                range.invalid_ids = invalid_ids;
                assert_eq!(range.invalid_ids, expected);
            }
        )*
        }
    }

    part_2_strategy_tests!{
        range_99_115: (99, 115, vec![ProductId::new(99), ProductId::new(111)]),
        range_11_22: (11, 22, vec![ProductId::new(11), ProductId::new(22)]),
        range_998_1012: (998, 1012, vec![ProductId::new(999), ProductId::new(1010)]),
        range_1188511880_1188511890: (1188511880, 1188511890, vec![ProductId::new(1188511885)]),
        range_222220_222224: (222220, 222224, vec![ProductId::new(222222)]),
    }
}
