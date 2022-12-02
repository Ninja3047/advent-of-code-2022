use std::collections::BinaryHeap;

fn solve(input: &str) -> usize {
    input
        .split("\n\n")
        .take_while(|p| !(*p).is_empty())
        .map(|elf| {
            elf.lines()
                .filter_map(|item| str::parse::<usize>(item).ok())
                .sum()
        })
        .collect::<BinaryHeap<usize>>()
        .iter()
        .take(3)
        .sum()
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
        let input = r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
"#;

        assert_eq!(solve(input), 45000);
    }
}
