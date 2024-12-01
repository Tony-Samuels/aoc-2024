use aoc_runner_derive::aoc;
use atoi_simd::parse;

// Perf notes:
// - Using `u32` and abs_diff is ~20-30% slower
// - Using `&str`, `parse` and `str::split` is ~20-30% slower
#[aoc(day1, part1)]
pub fn part1(input: &str) -> i32 {
    let input = input.as_bytes();

    let left = &mut Vec::with_capacity(1_000);
    let right = &mut Vec::with_capacity(1_000);

    for line in input.split(|&c| c == b'\n') {
        let iter = &mut line.split(|&c| c == b' ');
        let num1: i32 = parse(iter.next().unwrap()).unwrap();
        let num2: i32 = parse(iter.skip_while(|s| s.is_empty()).next().unwrap()).unwrap();

        debug_assert!(
            iter.next().is_none(),
            "Expected line to have only two numbers: {}",
            std::str::from_utf8(line).unwrap()
        );

        left.push(num1);
        right.push(num2);
    }

    left.sort_unstable();
    right.sort_unstable();

    left.iter()
        .zip(right.iter())
        .map(|(left, right)| (left - right).abs())
        .sum::<i32>()
}

#[cfg(test)]
mod tests {
    use super::part1;

    #[test]
    fn example() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";

        assert_eq!(part1(input), 11);
    }
}
