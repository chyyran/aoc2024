use std::collections::hash_map::Entry;
use aoc_runner_derive::aoc;

#[inline(always)]
fn parse_line(input: &str) -> (u32, u32) {
    let mut split = input.split("   ");
    let num1 = split.next().and_then(|f| f.parse().ok()).unwrap();
    let num2 = split.next().and_then(|f| f.parse().ok()).unwrap();

    (num1, num2)
}

#[aoc(day1, part1)]
pub fn part1(input: &str) -> u32 {
    let mut list1: Vec<u32> = Vec::with_capacity(1000);
    let mut list2: Vec<u32> = Vec::with_capacity(1000);

    let mut total_distance = 0;
    let lines = input.lines();

    for line in lines {
        let (num1, num2) = parse_line(line);
        list1.push(num1);
        list2.push(num2);
    }

    list1.sort();
    list2.sort();

    for (num1, num2) in list1.into_iter().zip(list2.into_iter()) {
        total_distance += num1.abs_diff(num2);
    }

    
    total_distance
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> u32 {
    let mut list1: Vec<u32> = Vec::with_capacity(1000);
    let mut list2 = rustc_hash::FxHashMap::default();
    let mut total_similarity = 0;

    let lines = input.lines();

    for line in lines {
        let (num1, num2) = parse_line(line);
        list1.push(num1);
        match list2.entry(num2) {
            Entry::Occupied(mut e) => {
                *e.get_mut() += 1;
            }
            Entry::Vacant(e) => {
                e.insert(1);
            }
        }
    }

    for id in list1 {
        total_similarity += id * list2.get(&id).unwrap_or(&0);
    }

    total_similarity
}
