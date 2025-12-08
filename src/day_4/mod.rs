use std::fs;
use std::fmt::Display;

use crate::day_4::render::Map;

pub mod render;



impl From<Vec<&str>> for Map {
    fn from(lines: Vec<&str>) -> Self {
        let size = lines.len() as u8;
        let mut diagram = Vec::new();
        for line in lines {
            for c in line.chars() {
                diagram.push(c);
            }
        }
        Map {
            diagram,
            size
        }
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.size {
            for x in 0..self.size {
                let index = (y as usize) * (self.size as usize) + (x as usize);
                write!(f, "{}", self.diagram[index])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Map {
    pub fn get(&self, x: u8, y: u8) -> Option<char> {
        if x >= self.size || y >= self.size {
            return None;
        }
        let index = (y as usize) * (self.size as usize) + (x as usize);
        Some(self.diagram[index])
    }
    pub fn set(&mut self, x: u8, y: u8, value: char) {
        if x >= self.size || y >= self.size {
            return;
        }
        let index = (y as usize) * (self.size as usize) + (x as usize);
        self.diagram[index] = value;
    }

    pub fn get_neighbors(&self, x: u8, y: u8) -> Vec<char> {
        if x >= self.size || y >= self.size {
            return vec![];
        }
        let mut neighbors = Vec::new();
        let directions = [(-1, -1), (0, -1), (1, -1),
                          (-1, 0),          (1, 0),
                          (-1, 1),  (0, 1),  (1, 1)];
        for (dx, dy) in directions.iter() {
            let nx = x as i16 + dx;
            let ny = y as i16 + dy;
            if nx >= 0 && ny >= 0 {
                if let Some(c) = self.get(nx as u8, ny as u8) {
                    neighbors.push(c);
                }
            }
        }
        neighbors
    }

    pub fn is_valid_position(&self, x: u8, y: u8, threshold: usize) -> bool {
        let current = self.get(x, y);
        if current.is_none() || (current.unwrap() != '@' && current.unwrap() != 'x') {
            return false
        }
        let neighbors = self.get_neighbors(x, y);
        let occupied_count = neighbors.iter().filter(|&&c| c == '@' || c == 'x').count();
        occupied_count < threshold
    }
}


pub fn day_4() {
    let contents = fs::read_to_string("./src/day_4/input.txt").expect("Could not read file");
    let lines: Vec<&str> = contents.lines().collect();
    let mut map = Map::from(lines);
    println!("Initial map:\n{}", map);
    let mut total = 0;

    while true {
        let mut count = 0;
        let mut positions_to_mark: Vec<(u8, u8)> = Vec::new();
        for y in 0..map.size {
            for x in 0..map.size {
                if map.is_valid_position(x, y, 4) {
                    map.set(x, y, 'x');
                    count += 1;
                    positions_to_mark.push((x, y));
                }
            }
        }
        for pos in positions_to_mark {
            map.set(pos.0, pos.1, '.');
        }
        println!("Updated map:\n{} {}", map, count);
        if count == 0 {
            break;
        }
        total += count;
    }
    println!("Total marked positions: {}", total);

   
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_from() {
        let lines = vec!["abc", "def"];
        let map = Map::from(lines);
        assert_eq!(map.diagram, vec!['a', 'b', 'c', 'd', 'e', 'f']);
    }

    #[test]
    fn test_map_get() {
        let lines = vec!["abc", "def", "ghi"];
        let map = Map::from(lines);
        assert_eq!(map.get(0, 0), Some('a'));
        assert_eq!(map.get(1, 0), Some('b'));
        assert_eq!(map.get(2, 1), Some('f'));
        assert_eq!(map.get(2, 2), Some('i'));
        assert_eq!(map.get(3, 0), None);
        assert_eq!(map.get(0, 3), None);
    }


    #[test]
    fn test_get_neighbors() {
        let lines = vec!["abc", "def", "ghi"];
        let map = Map::from(lines);
        assert_eq!(map.get_neighbors(0, 0), vec!['b', 'd', 'e']);
        assert_eq!(map.get_neighbors(1, 1), vec!['a', 'b', 'c', 'd', 'f', 'g', 'h', 'i']);
        assert_eq!(map.get_neighbors(3, 3), vec![]);
    }
}