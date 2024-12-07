use std::{
    cmp::Ordering,
    collections::{hash_map::Entry, HashMap, HashSet},
};

use aoc_runner_derive::aoc;
#[aoc(day5, part1)]
pub fn part1(input: &str) -> u32 {
    let (ordering, updates) = input.split_at(input.find("\n\n").unwrap());
    let ordering = PageOrdering::parse(ordering);
    let updates: Vec<Vec<u32>> = updates
        .trim()
        .lines()
        .map(|line| line.split(",").map(|p| p.parse::<u32>().unwrap()).collect())
        .filter(|s: &Vec<u32>| s.is_sorted_by(|a, b| ordering.sorted(a, b)))
        .collect();

    let mut middle_sum = 0;

    for update in updates {
        let midpoint = update.len() / 2;
        middle_sum += update[midpoint];
    }
    middle_sum
}

#[aoc(day5, part2)]
pub fn part2(input: &str) -> u32 {
    let (ordering, updates) = input.split_at(input.find("\n\n").unwrap());
    let ordering = PageOrdering::parse(ordering);
    let updates: Vec<Vec<u32>> = updates
        .trim()
        .lines()
        .map(|line| line.split(",").map(|p| p.parse::<u32>().unwrap()).collect())
        .filter(|s: &Vec<u32>| !s.is_sorted_by(|a, b| ordering.sorted(a, b)))
        .collect();

    let mut middle_sum = 0;

    for mut update in updates {
        update.sort_by(|a, b| ordering.sort(a, b));
        let midpoint = update.len() / 2;
        middle_sum += update[midpoint];
    }
    middle_sum
}
pub struct PageOrdering {
    orders: HashMap<u32, HashSet<u32>>,
}

impl PageOrdering {
    pub fn parse(input: &str) -> Self {
        let mut orders: HashMap<u32, HashSet<u32>> = HashMap::new();
        for line in input.lines() {
            let mut split = line.split("|");
            let before = split.next().and_then(|s| s.parse::<u32>().ok()).unwrap();
            let after = split.next().and_then(|s| s.parse::<u32>().ok()).unwrap();
            match orders.entry(before) {
                Entry::Occupied(mut occupied_entry) => {
                    occupied_entry.get_mut().insert(after);
                }
                Entry::Vacant(vacant_entry) => {
                    let mut set = HashSet::new();
                    set.insert(after);
                    vacant_entry.insert(set);
                }
            };
        }
        PageOrdering { orders }
    }

    pub fn sorted(&self, a: &u32, b: &u32) -> bool {
        // let orderings_a = self.orders.get(a);
        let orderings_b = self.orders.get(b);

        if orderings_b.is_none() {
            // Order free
            return true;
        }

        if let Some(order) = orderings_b {
            // Everything in the b set has to be to the left of a, so
            // if it contains a then its not ordered.
            if order.contains(a) {
                return false;
            }
        }

        // if let Some(order) = orderings_a {
        //     // Everything in the a set has to be to the left of a, so
        //     // if it contains a then its not ordered.
        //     if order.contains(a) {
        //         return false;
        //     }
        // }

        return true;
    }

    pub fn sort(&self, a: &u32, b: &u32) -> Ordering {
        let orderings_a = self.orders.get(a);
        let orderings_b = self.orders.get(b);

        if orderings_a.is_none() && orderings_b.is_none() {
            // Order free
            return Ordering::Equal;
        }

        if let Some(order) = orderings_b {
            // Everything in the b set has to be to the left of a, so
            // if it contains a then its not ordered.
            if order.contains(a) {
                return Ordering::Greater;
            }
        }

        if let Some(order) = orderings_a {
            // Everything in the b set has to be to the left of a, so
            // if it contains a then its not ordered.
            if order.contains(b) {
                return Ordering::Less;
            }
        }

        // if let Some(order) = orderings_a {
        //     // Everything in the a set has to be to the left of a, so
        //     // if it contains a then its not ordered.
        //     if order.contains(a) {
        //         return false;
        //     }
        // }

        return Ordering::Equal;
    }
}

#[cfg(test)]
mod test {
    use crate::day5::part2;

    use super::{part1, PageOrdering};

    #[test]
    fn parse_order() {
        let ordering = PageOrdering::parse(
            r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13"#,
        );

        assert!([75, 47, 61, 53, 29].is_sorted_by(|a, b| ordering.sorted(a, b)));
        assert!([97, 61, 53, 29, 13].is_sorted_by(|a, b| ordering.sorted(a, b)));
        assert!([75, 29, 13].is_sorted_by(|a, b| ordering.sorted(a, b)));

        assert!(![75, 97, 47, 61, 53].is_sorted_by(|a, b| ordering.sorted(a, b)));
        assert!(![61, 13, 29].is_sorted_by(|a, b| ordering.sorted(a, b)));
        assert!(![97, 13, 75, 29, 47].is_sorted_by(|a, b| ordering.sorted(a, b)));
    }

    #[test]
    fn part1_test() {
        let ordering = part1(
            r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#,
        );

        assert_eq!(ordering, 143);
    }

    #[test]
    fn part2_test() {
        let ordering = part2(
            r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#,
        );

        assert_eq!(ordering, 123);
    }
}
