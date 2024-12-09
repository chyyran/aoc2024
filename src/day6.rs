use std::{collections::{HashMap, HashSet}, fmt::Write};

use aoc_runner_derive::{aoc, aoc_generator};


#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
    // NorthEast,
    // NorthWest,
    // SouthEast,
    // SouthWest,
}

impl Direction {
    #[inline(always)]
    fn step_one(self, coords: (usize, usize), width: usize, height: usize) -> Option<(usize, usize)> {
        let (row, col) = coords;
        let transformed = match self {
            Direction::North => row.checked_sub(1).map(|row| (row, col)),
            Direction::South => row.checked_add(1).map(|row| (row, col)),
            Direction::East => col.checked_add(1).map(|col| (row, col)),
            Direction::West => col.checked_sub(1).map(|col| (row, col)),
            // Direction::NorthEast => row.checked_sub(1).zip(col.checked_sub(1)),
            // Direction::NorthWest => row.checked_sub(1).zip(col.checked_add(1)),
            // Direction::SouthEast => row.checked_add(1).zip(col.checked_sub(1)),
            // Direction::SouthWest => row.checked_add(1).zip(col.checked_add(1)),
        };

        transformed.filter(|&(row, col)| row < height && col < width)
    }

    fn turn_90_right(&self) -> Self {
        match &self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

#[derive(Clone)]
pub struct Grid {
    obstacles: HashSet<usize>,
    visited: HashMap<usize, HashSet<Direction>>,
    guard_position: usize,
    guard_orientation: Direction,
    width: usize,
    height: usize
}

pub struct VisitedGrid(Grid);

impl VisitedGrid {
    pub fn len(&self) -> u32 {
        self.0.visited.len() as u32
    }
}

impl Grid {
    pub fn parse(input: &str) -> Grid {
        let width = input.lines().next().unwrap().trim().len();
        let mut height = 0;

        let mut obstacles: HashSet<usize> = HashSet::new();
        let mut guard = None;
        // Can't be smart about it because the newlines throw off the indices.
        for line in input.lines() {
            for (col, cell) in line.trim().as_bytes().iter().enumerate() {
                if *cell == b'#' {
                    obstacles.insert(Self::coordinate_to_index_width(width, (height, col)));
                }

                if *cell == b'^' {
                    guard = Some(Self::coordinate_to_index_width(width, (height, col)));
                }
            }

            height += 1;
        }
    
        
        let visited = HashMap::from([(guard.unwrap(), HashSet::from([Direction::North]))]);

        Grid {
            obstacles,
            guard_position: guard.unwrap(),
            // guard always starts north
            guard_orientation: Direction::North,
            width,
            height,
            visited,
        }
    }

    
    #[inline(always)]
    pub fn index_to_coordinate(&self, index: usize) -> (usize, usize) {
        let column = index % self.width;
        let row = index / self.height;
        (row, column)
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

    pub fn drive_guard(mut self) -> VisitedGrid {
        let next_move = self.guard_orientation.step_one(self.index_to_coordinate(self.guard_position), self.width, self.height);
        let mut queue = vec![next_move];

        let mut stop_rot = 0;
        while let Some(Some(next_move)) = queue.pop() {
            // Stop the guard from going around in circles forever
            if stop_rot > 3 {
                break;
            }
            let next_index = self.coordinate_to_index(next_move);

            if self.obstacles.contains(&next_index) {
                // rotate 90 deg
                self.guard_orientation = self.guard_orientation.turn_90_right();
                let next_move = self.guard_orientation.step_one(self.index_to_coordinate(self.guard_position), self.width, self.height);
                queue.push(next_move);
                stop_rot += 1;
                continue;
            }

            // Mark current index as visited
            match self.visited.entry(next_index) {
                std::collections::hash_map::Entry::Occupied(mut occupied_entry) => {
                    occupied_entry.get_mut().insert(self.guard_orientation);
                },
                std::collections::hash_map::Entry::Vacant(vacant_entry) => {
                    vacant_entry.insert(HashSet::new()).insert(self.guard_orientation);
                },
            }

            // move the guard without changing orientation
            self.guard_position = next_index;
            queue.push(self.guard_orientation.step_one(self.index_to_coordinate(self.guard_position), self.width, self.height));
            stop_rot = 0;
        }

        VisitedGrid(self)
    }

    fn will_loop(mut self) -> bool {
        let next_move = self.guard_orientation.step_one(self.index_to_coordinate(self.guard_position), self.width, self.height);
        let mut queue = vec![next_move];

        let mut stop_rot = 0;
        while let Some(Some(next_move)) = queue.pop() {
            let next_index = self.coordinate_to_index(next_move);

            // We looped back so it works.
            if self.visited.get(&next_index).is_some_and(|visited| visited.contains(&self.guard_orientation)) {
                return true;
            }

            // Stop the guard from going around in circles forever at the same location
            if stop_rot > 3 {
                break;
            }

            if self.obstacles.contains(&next_index) {
                // rotate 90 deg
                self.guard_orientation = self.guard_orientation.turn_90_right();
                let next_move = self.guard_orientation.step_one(self.index_to_coordinate(self.guard_position), self.width, self.height);
                queue.push(next_move);
                stop_rot += 1;
                continue;
            }

            // Mark current index as visited
            match self.visited.entry(next_index) {
                std::collections::hash_map::Entry::Occupied(mut occupied_entry) => {
                    occupied_entry.get_mut().insert(self.guard_orientation);
                },
                std::collections::hash_map::Entry::Vacant(vacant_entry) => {
                    vacant_entry.insert(HashSet::new()).insert(self.guard_orientation);
                },
            }            // move the guard without changing orientation
            self.guard_position = next_index;
            queue.push(self.guard_orientation.step_one(self.index_to_coordinate(self.guard_position), self.width, self.height));
            stop_rot = 0;
        }

        false
    }

    fn find_obstructions(mut self) -> u32 {
        // populate visited
        let original = self.clone();
        let visited = self.drive_guard().0;
        let potential_obstacles = visited.visited;

        let mut count = 0;
        for (obstacle, _) in potential_obstacles {
            let mut test = original.clone();
            test.obstacles.insert(obstacle);
            if test.will_loop() {
                count += 1;
            }
        }
        count
    }

}


impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.height {
            for col in 0..self.width {
                let index = self.coordinate_to_index((row, col));
                if self.guard_position == index {
                    match self.guard_orientation {
                        Direction::North => f.write_char('^')?,
                        Direction::South => f.write_char('V')?,
                        Direction::East => f.write_char('>')?,
                        Direction::West => f.write_char('<')?,
                    }
                } else if self.obstacles.contains(&index) {
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

impl std::fmt::Display for VisitedGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.0.height {
            for col in 0..self.0.width {
                let index = self.0.coordinate_to_index((row, col));
                if self.0.visited.contains_key(&index){
                    f.write_char('X')?;
                } else if self.0.obstacles.contains(&index) {
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

#[aoc(day6, part1)]
pub fn part1(input: &str) -> u32 {
    let grid = Grid::parse(input).drive_guard();
    grid.len()
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> u32 {
    let grid = Grid::parse(input);
    grid.find_obstructions()
}

mod test {
    use crate::day6::Grid;


    const TEST_INPUT: &str = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;

    #[test]
    pub fn test_parse() {
        let grid = Grid::parse(TEST_INPUT);
        assert_eq!(grid.to_string().trim(), TEST_INPUT)
    }

    #[test]
    pub fn test_part1() {

        const EXPECTED: &str = r#"....#.....
....XXXXX#
....X...X.
..#.X...X.
..XXXXX#X.
..X.X.X.X.
.#XXXXXXX.
.XXXXXXX#.
#XXXXXXX..
......#X.."#;
        let grid: crate::day6::VisitedGrid = Grid::parse(TEST_INPUT).drive_guard();
        assert_eq!(grid.to_string().trim(), EXPECTED);
        assert_eq!(grid.len(), 41)

    }

    #[test]
    pub fn test_part2() {

        let grid = Grid::parse(TEST_INPUT);
        assert_eq!(grid.find_obstructions(), 6)

    }
}