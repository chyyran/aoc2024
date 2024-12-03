use aoc_runner_derive::aoc;
use arrayvec::ArrayVec;

#[inline(always)]
pub fn diff_in_range(diff: i32) -> bool {
    let abs = diff.abs();
    abs >= 1 && abs <= 3
}

#[inline(always)]
pub fn diff_same_monotonicity(diff_a: i32, diff_b: i32) -> bool {
    diff_a.signum() == diff_b.signum()
}

#[aoc(day2, part1)]
pub fn part1(input: &str) -> u32 {
    let mut safe_count = 0;
    let lines = input.lines();

    'report: for line in lines {
        let mut levels = line.split(" ").map(|s| s.parse::<i32>().unwrap());
     
        let p1 = levels.next().unwrap();
        let p2 = levels.next().unwrap();

        let mut current_diff = p1 - p2;
        if !diff_in_range(current_diff) {
            // unsafe
            continue 'report;
        }


        let mut current = p2;

        for level in levels {
            let new_diff = current - level;
            if !diff_in_range(new_diff) || !diff_same_monotonicity(current_diff, new_diff) {
                // unsafe
                continue 'report;
            }
           
            current_diff = new_diff;
            current = level;
        }

        safe_count += 1;
    }

    safe_count
}

#[inline(always)]
fn subset_is_good_until(subset: &[i32]) -> Option<usize> {
    if subset.len() <= 1 {
        return None
    }

    let p1 = subset[0];
    let p2 = subset[1];
    let mut current_diff = p1 - p2;

    if !diff_in_range(current_diff) {
        return Some(1);
    }

    let mut current = p2;

    for (index, level) in subset[2..].iter().enumerate() {
        let new_diff = current - level;
        if !diff_in_range(new_diff) || !diff_same_monotonicity(current_diff, new_diff) {
            return Some(index + 2)
        }
       
        current_diff = new_diff;
        current = *level;
    }

    None

}


#[aoc(day2, part2)]
pub fn part2(input: &str) -> u32 {
    let mut safe_count = 0;
    let lines = input.lines();

    // let mut unsafe_levels = Vec::with_capacity(1000);

    'report: for line in lines {
        let levels = line.split(" ").map(|s| s.parse::<i32>().unwrap());
     
        let mut vec = levels.collect::<ArrayVec<i32, 12>>();
        if let Some(index) = subset_is_good_until(&vec) {

            // check edges 
            if subset_is_good_until(&vec[1..]).is_none() {
                safe_count += 1;
                continue 'report;
            }

            if subset_is_good_until(&vec[..vec.len() - 1]).is_none() {
                safe_count += 1;
                continue 'report;
            }

            let value = vec.remove(index);
            if subset_is_good_until(&vec).is_none() {
                safe_count += 1;
                continue 'report;
            }

            vec.insert(index, value);
            let value = vec.remove(index - 1);

            if subset_is_good_until(&vec).is_none() {
                safe_count += 1;
                continue 'report;
            }

            vec.insert(index - 1, value);
            vec.remove(index + 1);

            if subset_is_good_until(&vec).is_none() {
                safe_count += 1;
                continue 'report;
            }
            continue 'report;
        }
       

        // println!("{} is safe", line);
        safe_count += 1;
    }




    safe_count
}

#[cfg(test)]
mod test {
    use crate::day2::{part1, part2};

    #[test]
    fn part1_test() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

        assert_eq!(part1(input), 2);
    }

    #[test]
    fn part2_test() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

        assert_eq!(part2(input), 4);
    }

    #[test]
    fn part2_edge_test() {
        let input = "48 46 47 49 51 54 56
1 1 2 3 4 5
1 2 3 4 5 5
5 1 2 3 4 5
1 4 3 2 1
1 6 7 8 9
1 2 3 4 3
9 8 7 6 7
7 10 8 10 11
29 28 27 25 26 25 22 20
";

        assert_eq!(part2(input), 10);
    }


    #[test]
    fn part1_test_bad() {
        let input = "1 2 4 6 8 9 13";

        assert_eq!(part1(input), 0);
    }
}
