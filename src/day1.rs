use aoc_runner_derive::aoc;

#[aoc(day1, part1)]
pub fn part1(input: &str) -> i32 {
    let left = &mut Vec::with_capacity(1_000);
    let right = &mut Vec::with_capacity(1_000);

    for line in input.lines() {
        let mut iter = line.split_ascii_whitespace();
        let num1: i32 = iter.next().unwrap().parse().unwrap();
        let num2: i32 = iter.next().unwrap().parse().unwrap();

        debug_assert!(
            iter.next().is_none(),
            "Expected line to have only two numbers: {line}"
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
