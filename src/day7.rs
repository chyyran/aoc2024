use aoc_runner_derive::aoc;
#[aoc(day7, part1)]
pub fn part1(input: &str) -> u64 {
    input
        .lines()
        .map(|line| parse_line(line))
        .filter_map(|(test, ops)| {
            if has_valid_permutation(test, &ops) {
                Some(test)
            } else {
                None
            }
        })
        .sum()
}

#[aoc(day7, part2)]
pub fn part2(input: &str) -> u64 {
    input
        .lines()
        .map(|line| parse_line(line))
        .filter_map(|(test, ops)| {
            if has_valid_permutation_concat(test, &ops) {
                Some(test)
            } else {
                None
            }
        })
        .sum()
}

fn parse_line(input: &str) -> (u64, Vec<u64>) {
    let (test, equation) = input.split_at(input.find(":").unwrap());
    let test = test.parse::<u64>().unwrap();
    let operands = equation
        .trim_start_matches(":")
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    (test, operands)
}

fn has_valid_permutation(test: u64, operands: &[u64]) -> bool {
    if operands.len() == 1 {
        return test == operands[0];
    }

    let mut queue = Vec::from([(operands[0], 1)]);

    while let Some((operand, neighbour)) = queue.pop() {
        let sum = operand + operands[neighbour];
        let product = operand * operands[neighbour];
        let next = neighbour + 1;
        if next == operands.len() {
            // no more, so this is a leaf node.
            if sum == test || product == test {
                return true;
            }
        } else {
            // not done
            queue.push((sum, next));
            queue.push((product, next));
       
        }
    }

    false
}

#[inline(always)]
fn concat(a: u64, b: u64) -> u64 {
    let digits_in_b = b.checked_ilog10().unwrap_or_default() + 1;
    (a * (10u64.pow(digits_in_b))) + b
}

fn has_valid_permutation_concat(test: u64, operands: &[u64]) -> bool {
    if operands.len() == 1 {
        return test == operands[0];
    }

    let mut queue = Vec::from([(operands[0], 1)]);

    while let Some((operand, neighbour)) = queue.pop() {
        let sum = operand + operands[neighbour];
        let product = operand * operands[neighbour];
        let concat = concat(operand, operands[neighbour]);

        let next = neighbour + 1;
        if next == operands.len() {
            // no more, so this is a leaf node.
            if sum == test || product == test || concat == test {
                return true;
            }
        } else {
            // not done
            if sum <= test {
                queue.push((sum, next));
            }

            if product <= test {
                queue.push((product, next));
            }

            if concat <= test {
                queue.push((concat, next));
            }
        }
    }

    false
}

#[cfg(test)]
mod test {
    use crate::day7::{has_valid_permutation, part1, part2};

    // 1708854043847 too low
    use super::{concat, parse_line};

    #[test]
    pub fn concat_test() {
        assert_eq!(concat(11, 11), 1111);
        assert_eq!(concat(12, 345), 12345);
    }

    #[test]
    pub fn test_part1() {
        let input = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;

        assert_eq!(part1(input), 3749);
    }

    #[test]
    pub fn test_part2() {
        let input = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;

        assert_eq!(part2(input), 3749);
    }
}
