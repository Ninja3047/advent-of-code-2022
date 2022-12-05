use std::collections::HashSet;

fn solve(input: &str) -> usize {
    input
        .lines()
        .map(|s| {
            let a: Vec<_> = s
                .split(",")
                .map(|e| {
                    e.split("-")
                        .filter_map(|n| str::parse::<usize>(n).ok())
                        .collect::<Vec<usize>>()
                })
                .map(|v| HashSet::from_iter(v[0]..v[1] + 1))
                .collect();
            let intersection: HashSet<usize> = &a[0] & &a[1];
            (intersection == a[0] || intersection == a[1]) as usize
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
        let input = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#;

        assert_eq!(solve(input), 2);
    }

    #[test]
    fn example_two() {
        let input = r#"2-4,6-8"#;

        assert_eq!(solve(input), 0);
    }

    #[test]
    fn example_three() {
        let input = r#"5-7,7-9"#;

        assert_eq!(solve(input), 0);
    }

    #[test]
    fn example_four() {
        let input = r#"6-6,4-6"#;

        assert_eq!(solve(input), 1);
    }

    #[test]
    fn example_five() {
        let input = r#"2-8,3-7"#;

        assert_eq!(solve(input), 1);
    }
}
