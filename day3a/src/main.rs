use std::collections::HashSet;

fn solve(input: &str) -> usize {
    input
        .lines()
        .map(|s| {
            let (left, right) = s.split_at(s.len() / 2);
            let left_set: HashSet<char> = HashSet::from_iter(left.chars());
            let right_set: HashSet<char> = HashSet::from_iter(right.chars());
            (&right_set & &left_set).into_iter().map(|c| {
                if c.is_ascii_uppercase() {
                    c as usize - 'A' as usize + 1 + 26
                } else {
                    c as usize - 'a' as usize + 1
                }
            }).sum::<usize>()
        }).sum()
}

pub fn main() {
    let total = solve(include_str!("input.txt"));
    println!("{total}");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let input = r#"vJrwpWtwJgWrhcsFMMfFFhFp"#;

        assert_eq!(solve(input), 16);
    }

    #[test]
    fn example_two() {
        let input = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"#;

        assert_eq!(solve(input), 54);
    }

    #[test]
    fn example_three() {
        let input = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
"#;

        assert_eq!(solve(input), 157);
    }
}
