fn solve(input: &str) -> usize {
    0
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
        let input = r#""#;
        assert_eq!(solve(input), 0);
    }
}
