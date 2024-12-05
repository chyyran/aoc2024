use aoc_runner_derive::aoc;
#[aoc(day5, part1)]
pub fn part1(input: &str) -> u32 {
    0
}

pub struct PageOrdering {}

impl PageOrdering {
    pub fn parse(input: &str) -> Self {
        for line in input.lines() {
            let mut split = line.split("|");
            let before = split.next().and_then(|s| s.parse::<usize>().ok()).unwrap();
            let after = split.next().and_then(|s| s.parse::<usize>().ok()).unwrap();
        }

        todo!()
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn parse_order() {}
}
