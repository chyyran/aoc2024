use aoc_runner_derive::aoc;
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    fmt::Write,
    hash::Hash,
};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

impl Direction {
    #[inline(always)]
    fn step(
        self,
        coords: (usize, usize),
        width: usize,
        height: usize,
        step: usize,
    ) -> Option<(usize, usize)> {
        let (row, col) = coords;
        let transformed = match self {
            Direction::North => row.checked_sub(step).map(|row| (row, col)),
            Direction::South => row.checked_add(step).map(|row| (row, col)),
            Direction::East => col.checked_add(step).map(|col| (row, col)),
            Direction::West => col.checked_sub(step).map(|col| (row, col)),
            Direction::NorthEast => row.checked_sub(step).zip(col.checked_sub(step)),
            Direction::NorthWest => row.checked_sub(step).zip(col.checked_add(step)),
            Direction::SouthEast => row.checked_add(step).zip(col.checked_sub(step)),
            Direction::SouthWest => row.checked_add(step).zip(col.checked_add(step)),
        };

        transformed.filter(|&(row, col)| row < height && col < width)
    }
}

pub struct Map {
    towers: HashMap<usize, u8>,
    tower_index: HashMap<u8, Vec<usize>>,

    width: usize,
    height: usize,
    antinodes: HashSet<usize>,
}

pub struct AntinodeMap(Map);

impl AntinodeMap {
    pub fn len(&self) -> usize {
        self.0.antinodes.len()
    }
}

impl Map {
    pub fn len(&self) -> usize {
        self.height * self.width
    }

    pub fn parse(input: &str) -> Map {
        let width = input.lines().next().unwrap().trim().len();
        let mut height = 0;

        let mut towers = HashMap::new();
        let mut tower_index: HashMap<u8, Vec<_>> = HashMap::new();
        let mut antinodes = HashSet::new();

        // Can't be smart about it because the newlines throw off the indices.
        for line in input.lines() {
            for (col, cell) in line.trim().as_bytes().iter().enumerate() {
                if *cell != b'.' {
                    let index = Self::coordinate_to_index_width(width, (height, col));
                    towers.insert(index, *cell);
                    match tower_index.entry(*cell) {
                        std::collections::hash_map::Entry::Occupied(mut occupied_entry) => {
                            occupied_entry.get_mut().push(index);
                        }
                        std::collections::hash_map::Entry::Vacant(vacant_entry) => {
                            vacant_entry.insert(vec![index]);
                        }
                    }
                }
            }

            height += 1;
        }

        Map {
            towers,
            width,
            height,
            antinodes,
            tower_index,
        }
    }

    pub fn calculate_antinodes_part1(mut self) -> AntinodeMap {
        let len = self.len();
        for (frequency, indices) in self.tower_index.iter() {
            if indices.len() == 1 {
                continue;
            }
            let pairs: Vec<_> = indices
                .iter()
                .map(|index| self.index_to_coordinate(*index))
                .combinations(2)
                .collect();
            for pair in pairs {
                let ((y1, x1), (y2, x2)) = (pair[0], pair[1]);

                let y_distance = y2 - y1;
                let x_distance = x2 - x1;

                let antinode_1 = ((y1 - y_distance), (x1 - x_distance));
                let antinode_2 = ((y2 + y_distance), (x2 + x_distance));

                if antinode_1.0 >= 0 && antinode_1.1 >= 0 && antinode_1.0 < (self.height as i32) && antinode_1.1 < (self.width as i32){
                    let antinode_1 =
                        self.coordinate_to_index((antinode_1.0 as usize, antinode_1.1 as usize));
                    self.antinodes.insert(antinode_1);
                }

                if antinode_2.0 >= 0 && antinode_2.1 >= 0 && antinode_2.0 < (self.height as i32) && antinode_2.1 < (self.width as i32) {
                    let antinode_2 =
                        self.coordinate_to_index((antinode_2.0 as usize, antinode_2.1 as usize));
                    self.antinodes.insert(antinode_2);
                }
            }
        }

        AntinodeMap(self)
    }

    pub fn calculate_antinodes_part2(mut self) -> AntinodeMap {
        let len = self.len();
        for (frequency, indices) in self.tower_index.iter() {
            if indices.len() == 1 {
                continue;
            }

            // All towers with more than 1 are antinodes
            self.antinodes.extend(indices.iter());

            let pairs: Vec<_> = indices
                .iter()
                .map(|index| self.index_to_coordinate(*index))
                .combinations(2)
                .collect();

            for pair in pairs {
                let ((y1, x1), (y2, x2)) = (pair[0], pair[1]);

                let y_distance = y2 - y1;
                let x_distance = x2 - x1;

                let mut antinode_1 = ((y1 - y_distance), (x1 - x_distance));
                while antinode_1.0 >= 0 && antinode_1.1 >= 0 && antinode_1.0 < (self.height as i32) && antinode_1.1 < (self.width as i32){
                    self.antinodes.insert(self.coordinate_to_index((antinode_1.0 as usize, antinode_1.1 as usize)));
                    antinode_1 = (antinode_1.0 - y_distance, antinode_1.1 - x_distance);
                }

                let mut antinode_2 = ((y2 + y_distance), (x2 + x_distance));

                while antinode_2.0 >= 0 && antinode_2.1 >= 0 && antinode_2.0 < (self.height as i32) && antinode_2.1 < (self.width as i32){
                    self.antinodes.insert(self.coordinate_to_index((antinode_2.0 as usize, antinode_2.1 as usize)));
                    antinode_2 = (antinode_2.0 + y_distance, antinode_2.1 + x_distance);
                }

            }
        }

        AntinodeMap(self)
    }


    #[inline(always)]
    pub fn index_to_coordinate(&self, index: usize) -> (i32, i32) {
        // convert into f32 to avoid precision loss
        let column = index % self.width;
        let row = index / self.height;
        (row as i32, column as i32)
    }

    #[inline(always)]
    pub fn coordinate_to_index(&self, coords: (usize, usize)) -> usize {
        let (row, column) = coords;
        let index = row * self.width + column;
        index
    }

    #[inline(always)]
    pub fn coordinate_to_index_width(width: usize, coords: (usize, usize)) -> usize {
        let (row, column) = coords;
        let index = row * width + column;
        index
    }
}

#[aoc(day8, part1)]
pub fn part1(input: &str) -> u32 {
    // 365 is too high
    Map::parse(input).calculate_antinodes_part1().len() as u32
}

#[aoc(day8, part2)]
pub fn part2(input: &str) -> u32 {
    // 365 is too high
    Map::parse(input).calculate_antinodes_part2().len() as u32
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.height {
            for col in 0..self.width {
                let index = self.coordinate_to_index((row, col));
                if let Some(tower) = self.towers.get(&index) {
                    f.write_char(*tower as char)?;
                } else {
                    f.write_char('.')?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl std::fmt::Display for AntinodeMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.0.height {
            for col in 0..self.0.width {
                let index = self.0.coordinate_to_index((row, col));
                if let Some(tower) = self.0.towers.get(&index) {
                    f.write_char(*tower as char)?;
                } else if self.0.antinodes.contains(&index) {
                    f.write_char('#')?;
                } else {
                    f.write_char('.')?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::day8::Map;

    const TEST_INPUT: &str = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#;

    #[test]
    fn test_parse() {
        assert_eq!(TEST_INPUT, Map::parse(TEST_INPUT).to_string().trim())
    }

    #[test]
    fn test_part1() {
        const EXPECTED: &str = r#"......#....#
...#....0...
....#0....#.
..#....0....
....0....#..
.#....A.....
...#........
#......#....
........A...
.........A..
..........#.
..........#."#;

        let map = Map::parse(TEST_INPUT).calculate_antinodes_part1();

        assert_eq!(map.len(), 14);
        assert_eq!(map.to_string().trim(), EXPECTED.trim());

    }

    #[test]
    fn test_part2() {
        const EXPECTED: &str = r#"##....#....#
.#.#....0...
..#.#0....#.
..##...0....
....0....#..
.#...#A....#
...#..#.....
#....#.#....
..#.....A...
....#....A..
.#........#.
...#......##"#;

        let map = Map::parse(TEST_INPUT).calculate_antinodes_part2();

        assert_eq!(map.len(), 34);
        assert_eq!(map.to_string().trim(), EXPECTED.trim());

    }
}
