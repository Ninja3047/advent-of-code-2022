#![feature(iter_array_chunks)]
use std::collections::HashSet;

fn solve(input: &str) -> usize {
    input
        .lines()
        .array_chunks::<3>()
        .map(|g| {
            g.map(|s| s.chars().collect::<HashSet<char>>())
                .into_iter()
                .reduce(|s, acc| &s & &acc)
                .unwrap()
                .into_iter()
                .map(|c| {
                    if c.is_ascii_uppercase() {
                        c as usize - 'A' as usize + 1 + 26
                    } else {
                        c as usize - 'a' as usize + 1
                    }
                })
                .sum::<usize>()
        })
        .sum()
}

fn main() {
    let total = solve(include_str!("input.txt"));
    println!("{total}");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let input = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#;

        assert_eq!(solve(input), 70);
    }
}
