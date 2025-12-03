#[derive(PartialEq, Debug)]
pub struct ProductId {
    pub id: u128,
}

impl Into<u128> for ProductId {
    fn into(self) -> u128 {
        self.id
    }
}

impl ProductId {
    pub fn new(id: u128) -> Self {
        ProductId { id }
    }

    pub fn get_halves(&self, offset_odd: bool) -> (String, String) {
        let str_id = self.id.to_string();
        let mid = if str_id.len() % 2 == 1 && offset_odd { str_id.len() / 2  + 1} else { str_id.len() / 2  };
        let (first_half, second_half) = str_id.split_at(mid);
        (first_half.to_string(), second_half.to_string())
    }

    pub fn is_valid(&self) -> bool {
        let str_id = self.id.to_string();
        if str_id.len() % 2 == 1 { return true}
        let halves = self.get_halves(false);
        return halves.0 != halves.1;
        
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_product_id_validity() {
        let valid_id = ProductId::new(1);
        let invalid_id = ProductId::new(11);
        let valid_id_two_digits = ProductId::new(12);
        assert!(valid_id.is_valid());
        assert!(!invalid_id.is_valid());
        assert!(valid_id_two_digits.is_valid())
    }

}