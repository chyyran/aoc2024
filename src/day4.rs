use std::{
    collections::HashSet,
    fmt::{Display, Write},
};

use aoc_runner_derive::{aoc, aoc_generator};

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Letter {
    Null = 0xA,
    X = 0x58,
    M = 0x4D,
    A = 0x41,
    S = 0x53,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
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
    fn transform(self, coords: (usize, usize), bounds: usize) -> Option<(usize, usize)> {
        let (row, col) = coords;
        let transformed = match self {
            Direction::North => row.checked_sub(1).map(|row| (row, col)),
            Direction::South => row.checked_add(1).map(|row| (row, col)),
            Direction::East => col.checked_add(1).map(|col| (row, col)),
            Direction::West => col.checked_sub(1).map(|col| (row, col)),
            Direction::NorthEast => row.checked_sub(1).zip(col.checked_sub(1)),
            Direction::NorthWest => row.checked_sub(1).zip(col.checked_add(1)),
            Direction::SouthEast => row.checked_add(1).zip(col.checked_sub(1)),
            Direction::SouthWest => row.checked_add(1).zip(col.checked_add(1)),
        };

        transformed.filter(|&(row, col)| row < bounds && col < bounds)
    }
}

impl From<u8> for Letter {
    fn from(value: u8) -> Self {
        match value {
            b'X' => Letter::X,
            b'M' => Letter::M,
            b'A' => Letter::A,
            b'S' => Letter::S,
            _ => Letter::Null,
        }
    }
}

impl Display for Letter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Letter::Null => f.write_char('.'),
            Letter::X => f.write_char('X'),
            Letter::M => f.write_char('M'),
            Letter::A => f.write_char('A'),
            Letter::S => f.write_char('S'),
        }
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.letters.chunks(self.size) {
            for letter in line {
                match letter.letter {
                    Letter::Null => f.write_char('.')?,
                    Letter::X => f.write_char('X')?,
                    Letter::M => f.write_char('M')?,
                    Letter::A => f.write_char('A')?,
                    Letter::S => f.write_char('S')?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

struct Grid {
    letters: Vec<Cell>,
    // grid is square
    size: usize,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
struct Cell {
    pub letter: Letter,
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Grid {
    Grid::parse(input)
}

impl Grid {
    pub fn len(&self) -> usize {
        self.letters.len()
    }

    pub fn parse(input: &str) -> Grid {
        // SAFETY: `Letter` is constructed such that it is transmutable from bytes, where newlines are the 'Null' character.
        // This makes parsing extremely fast.
        let mut letters = Vec::from(unsafe { std::mem::transmute::<_, &[Cell]>(input.as_bytes()) });
        let size = letters
            .iter()
            .enumerate()
            .find(|(_, s)| s.letter == Letter::Null)
            .unwrap()
            .0;

        letters.retain(|s| s.letter != Letter::Null);
        Grid { letters, size }
    }

    #[inline(always)]
    pub fn at_index_mut(&mut self, index: usize) -> Option<&mut Cell> {
        self.letters.get_mut(index)
    }

    #[inline(always)]
    pub fn at_index(&self, index: usize) -> Option<&Cell> {
        self.letters.get(index)
    }

    #[inline(always)]
    pub fn index_to_coordinate(&self, index: usize) -> (usize, usize) {
        let column = index % self.size;
        let row = index / self.size;
        (row, column)
    }

    #[inline(always)]
    pub fn coordinate_to_index(&self, coords: (usize, usize)) -> usize {
        let (row, column) = coords;
        let index = row * self.size + column;
        index
    }

    #[inline(always)]
    pub fn at_direction(
        &self,
        index: usize,
        direction: Direction,
    ) -> Option<(&Cell, usize, Direction)> {
        direction
            .transform(self.index_to_coordinate(index), self.size)
            .map(|c| self.coordinate_to_index(c))
            .and_then(|i| self.at_index(i).map(|s| (s, i, direction)))
    }
}

#[aoc(day4, part1)]
pub fn part1(grid: &Grid) -> u32 {
    const ALL_DIRECTIONS: &[Direction] = &[
        Direction::North,
        Direction::South,
        Direction::East,
        Direction::West,
        Direction::NorthEast,
        Direction::NorthWest,
        Direction::SouthEast,
        Direction::SouthWest,
    ];

    let mut valid_xs = 0;

    for x_index in 0..grid.len() {
        let Some(cell) = grid.at_index(x_index) else {
            continue;
        };

        if cell.letter != Letter::X {
            continue;
        }

        let ms = ALL_DIRECTIONS
            .iter()
            .map(|d| grid.at_direction(x_index, *d))
            .filter_map(|s| s)
            .filter_map(|(s, m_index, direction)| {
                if s.letter == Letter::M {
                    Some((m_index, direction))
                } else {
                    None
                }
            });

        for (m_index, direction) in ms {
            let Some((cell, a_index, _)) = grid.at_direction(m_index, direction) else {
                continue;
            };

            if cell.letter != Letter::A {
                continue;
            }

            let Some((cell, _s_index, _)) = grid.at_direction(a_index, direction) else {
                continue;
            };

            if cell.letter != Letter::S {
                continue;
            }

            valid_xs += 1;
        }
    }

    valid_xs
}

#[aoc(day4, part2)]
pub fn part2(grid: &Grid) -> u32 {
    let mut valid_as = 0;

    for a_index in 0..grid.len() {
        let Some(cell) = grid.at_index(a_index) else {
            continue;
        };

        if cell.letter != Letter::A {
            continue;
        }

        // Check NorthEast corner
        let Some((cell, _index, _)) = grid.at_direction(a_index, Direction::NorthEast) else {
            continue;
        };

        if cell.letter != Letter::M && cell.letter != Letter::S {
            continue;
        }

        // if NE is M, then SW must be S
        if cell.letter == Letter::M
            && !grid
                .at_direction(a_index, Direction::SouthWest)
                .is_some_and(|(cell, _, _)| cell.letter == Letter::S)
        {
            continue;
        } else if cell.letter == Letter::S
            && !grid
                .at_direction(a_index, Direction::SouthWest)
                .is_some_and(|(cell, _, _)| cell.letter == Letter::M)
        {
            continue;
        }

        // KNOWN
        // M  ?       S  ?
        //   A    OR   A
        // ?  S      ?  M

        // Check NorthWest corner
        let Some((cell, _index, _)) = grid.at_direction(a_index, Direction::NorthWest) else {
            continue;
        };

        if cell.letter != Letter::M && cell.letter != Letter::S {
            continue;
        } else if cell.letter == Letter::M
            && !grid
                .at_direction(a_index, Direction::SouthEast)
                .is_some_and(|(cell, _, _)| cell.letter == Letter::S)
        {
            continue;
        }

        // if NW is S, then SE must be M
        if cell.letter == Letter::S
            && !grid
                .at_direction(a_index, Direction::SouthEast)
                .is_some_and(|(cell, _, _)| cell.letter == Letter::M)
        {
            continue;
        }

        valid_as += 1;
    }

    valid_as
}
#[cfg(test)]
mod test {
    use crate::day4::{part1, part2, Direction, Grid, Letter};

    #[test]
    pub fn part1_lib_test() {
        assert_eq!(Direction::South.transform((3, 9), 10), Some((4, 9)));

        const INPUT: &str = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
"#;

        let grid = Grid::parse(INPUT);
        assert_eq!(grid.to_string(), INPUT);
        assert_eq!(grid.index_to_coordinate(0), (0, 0));
        assert_eq!(grid.index_to_coordinate(1), (0, 1));
        assert_eq!(grid.index_to_coordinate(10), (1, 0));
        assert_eq!(grid.index_to_coordinate(11), (1, 1));
        assert_eq!(grid.index_to_coordinate(20), (2, 0));
        assert_eq!(grid.coordinate_to_index((2, 0)), 20);
        assert_eq!(grid.coordinate_to_index((1, 1)), 11);
        assert_eq!(grid.coordinate_to_index((0, 1)), 1);

        assert_eq!(grid.coordinate_to_index((0, 1)), 1);
        assert_eq!(grid.coordinate_to_index((0, 5)), 5);

        assert_eq!(Direction::East.transform((0, 5), 10), Some((0, 6)));

        assert_eq!(
            grid.at_direction(grid.coordinate_to_index((3, 9)), Direction::South)
                .unwrap()
                .0
                .letter,
            Letter::M
        );

        assert_eq!(
            grid.at_direction(grid.coordinate_to_index((0, 4)), Direction::East)
                .unwrap()
                .0
                .letter,
            Letter::X
        );
    }

    #[test]
    pub fn part1_test() {
        const INPUT: &str = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
"#;

        assert_eq!(part1(INPUT), 18);
    }

    #[test]
    pub fn part2_test() {
        const INPUT: &str = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
"#;

        assert_eq!(part2(INPUT), 9);
    }

    #[test]
    pub fn part1_test2() {
        const INPUT: &str = r#"SAMSXXMAXAMX
SAASMMASAXMA
SMMSXSAXMSMS
XMASAMASXXAA
SASMAMSMMMMM
ASMSMMMAAMMA
MAXAMAMMMSMA
XMSMSAXMXXMS
XMAMAAMMMMXX
SSSMSMMAAXAS
AAAMAASXMMMS
MMMMMXMAAXAS
"#;

        assert_eq!(part1(INPUT), 8);
    }
}

// X...S..
// .....A.
// SSS.S.M
// AAA.A.S
// M.MMM..
// X.X.XMA
