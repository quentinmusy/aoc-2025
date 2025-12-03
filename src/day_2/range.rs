use std::{convert::From, fmt::Display};
use crate::day_2::product_id::ProductId;

pub struct Range {
    pub start: ProductId,
    pub end: ProductId,
    pub invalid_ids: Vec<ProductId>,
}

impl Display for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}, invalids: {:?}", self.start.id, self.end.id, self.invalid_ids.iter().map(|id| id.id).collect::<Vec<u128>>())
    }
}

trait InvalidIdStrategy {
    fn find_invalid_ids(&mut self);
}

impl Range {
    fn new(start: ProductId, end: ProductId) -> Self {
        Self {start, end, invalid_ids: Vec::new()}
    }

    /**
     * We don't want to check every ID in the range, as ranges can be quite big,
     * so we can try to find possible invalid IDs based on their structure.
     * We know that invalid IDs have the same digits in both halves.
     * So we need to look at first half of first ID and last half of last ID
     * to find possible candidates.
     */
    pub fn find_invalid_ids(&mut self) {
        let (start_first_half, _) = self.start.get_halves(false);
        let (start_second_half, _) = self.end.get_halves(true);
    
        // Compute all possible "doubles" in the range
        // And check if they are within the range
        let start = start_first_half.parse::<u128>().unwrap_or(0);
        let end = start_second_half.parse::<u128>().unwrap_or(0);
        for half in std::cmp::min(start, end)..=std::cmp::max(start, end) {
            let half_str = half.to_string();
            let possible_id_str = format!("{}{}", half_str, half_str);
            if let Ok(possible_id) = possible_id_str.parse::<u128>() {
                if possible_id >= self.start.id && possible_id <= self.end.id {
                    self.invalid_ids.push(ProductId::new(possible_id));
                }
            }
        }
    }
}

impl From<&str> for Range {
    fn from(s: &str) -> Self {
        let (start_str, end_str) = s.split_at(s.find('-').expect("Invalid range format"));
        let start_id = start_str.parse::<u128>().expect("Invalid start ID");
        let end_id = end_str[1..].parse::<u128>().expect("Invalid end ID");
        Range::new(ProductId::new(start_id), ProductId::new(end_id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! find_invalid_ids_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (start_number, end_number, expected) = $value;
                let start = ProductId::new(start_number);
                let end = ProductId::new(end_number);
                let mut range = Range::new(start, end);
                range.find_invalid_ids();
                assert_eq!(range.invalid_ids, expected);
            }
        )*
        }
    }

    find_invalid_ids_tests!{
        range_99_115: (99, 115, vec![ProductId::new(99)]),
        range_11_22: (11, 22, vec![ProductId::new(11), ProductId::new(22)]),
        range_998_1012: (998, 1012, vec![ProductId::new(1010)]),
        range_1188511880_1188511890: (1188511880, 1188511890, vec![ProductId::new(1188511885)]),
        range_222220_222224: (222220, 222224, vec![ProductId::new(222222)]),
        range_868_1423: (868, 1423, vec![ProductId::new(1010), ProductId::new(1111), ProductId::new(1212), ProductId::new(1313), ProductId::new(1414)]),
        range_6968_10197: (6968, 10197, vec![ProductId::new(6969), ProductId::new(7070), ProductId::new(7171), ProductId::new(7272), ProductId::new(7373), ProductId::new(7474), ProductId::new(7575), ProductId::new(7676), ProductId::new(7777), ProductId::new(7878), ProductId::new(7979), ProductId::new(8080), ProductId::new(8181), ProductId::new(8282), 
        ProductId::new(8383), ProductId::new(8484), ProductId::new(8585), ProductId::new(8686), ProductId::new(8787), ProductId::new(8888), ProductId::new(8989), ProductId::new(9090), ProductId::new(9191), ProductId::new(9292), ProductId::new(9393), ProductId::new(9494), ProductId::new(9595), ProductId::new(9696), ProductId::new(9797), ProductId::new(9898), ProductId::new(9999)]),
    }
    


}