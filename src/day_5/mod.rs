use std::fs;

#[derive(Debug, Clone, Copy)]
struct IngredientRange {
    min: u64,
    max: u64
}

impl IngredientRange {
    pub fn contains(&self, value: u64) -> bool {
        value >= self.min && value <= self.max
    }


    pub fn overlaps(&self, other: &IngredientRange) -> bool {
        self.min <= other.max && other.min <= self.max
    }

    pub fn merge(&mut self, other: &IngredientRange) {
        self.min = self.min.min(other.min);
        self.max = self.max.max(other.max);
       
    }

    pub fn total_fresh(&self) -> u64 {
        self.max - self.min + 1
    }
}

impl From<&str> for IngredientRange {
    fn from(s: &str) -> Self {
        let parts: Vec<&str> = s.split('-').collect();
        IngredientRange {
            min: parts[0].parse::<u64>().unwrap(),
            max: parts[1].parse::<u64>().unwrap()
        }
    }
}


pub fn day_5() {
    let contents = fs::read_to_string("./src/day_5/input.txt").expect("Could not read file");
    let mut ranges: Vec<IngredientRange> = Vec::new();
    let mut count = 0;
    let mut objects_to_count: Vec<u64> = Vec::new();
    contents.lines().for_each(|x|
        if x.contains("-") {
                ranges.push(IngredientRange::from(x));
        }

        else {
            let maybe_number = x.parse::<u64>();
            if maybe_number.is_ok() {
                let number = maybe_number.unwrap();
                objects_to_count.push(number);                
            }
        }
    );

    ranges.sort_by(|a, b| a.min.cmp(&b.min));

    let mut final_ranges: Vec<IngredientRange> = Vec::new();

    let mut current_range = ranges[0].clone();
    for range in ranges.iter_mut().skip(1) {
        if current_range.overlaps(range) {
            current_range.merge(range);
        } else {
            final_ranges.push(current_range);
            current_range = range.clone();
        }
    }
    final_ranges.push(current_range);


    for number in objects_to_count {
        let mut is_valid = false;
        for range in &final_ranges {
            if range.contains(number) {
                is_valid = true;
                break;
            }
        }
        if is_valid {
            count += 1;
        }
    }

    println!("Valid ingredient quantities: {}, ranges: {:?}", count, ranges);

    let total= final_ranges.iter().fold(0, |total, range | total + range.total_fresh());
    println!("Total fresh ingredient quantities: {}, total valid {}", count, total);
}