// use std::{
//     collections::HashSet,
//     fmt::{Display, Write},
// };

// use aoc_runner_derive::{aoc, aoc_generator};

// #[repr(u8)]
// #[derive(Debug, Copy, Clone, Eq, PartialEq)]
// enum Letter {
//     Null = 0xA,
//     Space = b'.',
//     Block = b'#',
//     Visited = b'X',
// }

// #[derive(Debug, Copy, Clone, Eq, PartialEq)]
// enum Direction {
//     North,
//     South,
//     East,
//     West,
//     NorthEast,
//     NorthWest,
//     SouthEast,
//     SouthWest,
// }

// impl Direction {
//     #[inline(always)]
//     fn transform(self, coords: (usize, usize), bounds: usize) -> Option<(usize, usize)> {
//         let (row, col) = coords;
//         let transformed = match self {
//             Direction::North => row.checked_sub(1).map(|row| (row, col)),
//             Direction::South => row.checked_add(1).map(|row| (row, col)),
//             Direction::East => col.checked_add(1).map(|col| (row, col)),
//             Direction::West => col.checked_sub(1).map(|col| (row, col)),
//             Direction::NorthEast => row.checked_sub(1).zip(col.checked_sub(1)),
//             Direction::NorthWest => row.checked_sub(1).zip(col.checked_add(1)),
//             Direction::SouthEast => row.checked_add(1).zip(col.checked_sub(1)),
//             Direction::SouthWest => row.checked_add(1).zip(col.checked_add(1)),
//         };

//         transformed.filter(|&(row, col)| row < bounds && col < bounds)
//     }
// }

// // impl From<u8> for Letter {
// //     fn from(value: u8) -> Self {
// //         match value {
// //             b'X' => Letter::X,
// //             b'M' => Letter::M,
// //             b'A' => Letter::A,
// //             b'S' => Letter::S,
// //             _ => Letter::Null,
// //         }
// //     }
// // }

// // impl Display for Letter {
// //     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
// //         match self {
// //             Letter::Null => f.write_char('.'),
// //             Letter::X => f.write_char('X'),
// //             Letter::M => f.write_char('M'),
// //             Letter::A => f.write_char('A'),
// //             Letter::S => f.write_char('S'),
// //         }
// //     }
// // }

// // impl Display for Grid {
// //     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
// //         for line in self.letters.chunks(self.size) {
// //             for letter in line {
// //                 match letter {
// //                     Letter::Null => f.write_char('.')?,
// //                     Letter::X => f.write_char('X')?,
// //                     Letter::M => f.write_char('M')?,
// //                     Letter::A => f.write_char('A')?,
// //                     Letter::S => f.write_char('S')?,
// //                 }
// //             }
// //             writeln!(f)?;
// //         }
// //         Ok(())
// //     }
// // }

// struct Grid {
//     letters: Vec<Letter>,
//     // grid is square
//     size: usize,
// }

// #[aoc_generator(day6)]
// pub fn input_generator(input: &str) -> Grid {
//     Grid::parse(input)
// }

// impl Grid {
//     pub fn len(&self) -> usize {
//         self.letters.len()
//     }

//     pub fn parse(input: &str) -> Grid {
//         // SAFETY: `Letter` is constructed such that it is transmutable from bytes, where newlines are the 'Null' character.
//         // This makes parsing extremely fast.
//         let mut letters = Vec::from(unsafe { std::mem::transmute::<_, &[Letter]>(input.as_bytes()) });
//         let size = letters
//             .iter()
//             .enumerate()
//             .find(|(_, s)| **s == Letter::Null)
//             .unwrap()
//             .0;

//         Grid { letters, size }
//     }

//     #[inline(always)]
//     pub fn at_index_mut(&mut self, index: usize) -> Option<&mut Cell> {
//         self.letters.get_mut(index)
//     }

//     #[inline(always)]
//     pub fn at_index(&self, index: usize) -> Option<&Cell> {
//         self.letters.get(index)
//     }

//     #[inline(always)]
//     pub fn index_to_coordinate(&self, index: usize) -> (usize, usize) {
//         let column = index % self.size;
//         let row = index / self.size;
//         (row, column)
//     }

//     #[inline(always)]
//     pub fn coordinate_to_index(&self, coords: (usize, usize)) -> usize {
//         let (row, column) = coords;
//         let index = row * self.size + column;
//         index
//     }

//     #[inline(always)]
//     pub fn at_direction(
//         &self,
//         index: usize,
//         direction: Direction,
//     ) -> Option<(&Cell, usize, Direction)> {
//         direction
//             .transform(self.index_to_coordinate(index), self.size)
//             .map(|c| self.coordinate_to_index(c))
//             .and_then(|i| self.at_index(i).map(|s| (s, i, direction)))
//     }
// }

// #[cfg(test)]
// mod test {

// }
