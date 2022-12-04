#[derive(PartialEq, Eq, Ord, PartialOrd, Debug, Copy, Clone)]
enum RPS {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(PartialEq, Eq, Ord, PartialOrd, Debug, Copy, Clone)]
enum Outcome {
    Lose = 0,
    Draw = 3,
    Win = 6,
}

fn get_choice(opponent_choice: RPS, desired_outcome: Outcome) -> RPS {
    match (opponent_choice, desired_outcome) {
        (RPS::Rock, Outcome::Win) => RPS::Paper,
        (RPS::Rock, Outcome::Draw) => RPS::Rock,
        (RPS::Rock, Outcome::Lose) => RPS::Scissors,
        (RPS::Paper, Outcome::Win) => RPS::Scissors,
        (RPS::Paper, Outcome::Draw) => RPS::Paper,
        (RPS::Paper, Outcome::Lose) => RPS::Rock,
        (RPS::Scissors, Outcome::Win) => RPS::Rock,
        (RPS::Scissors, Outcome::Draw) => RPS::Scissors,
        (RPS::Scissors, Outcome::Lose) => RPS::Paper,
    }
}

fn solve(input: &str) -> usize {
    input
        .lines()
        .map(|s| {
            let (left, right) = s.split_once(" ").unwrap();
            let opponent_choice = match left {
                "A" => RPS::Rock,
                "B" => RPS::Paper,
                "C" => RPS::Scissors,
                _ => unreachable!("Error in input"),
            };

            let my_outcome = match right {
                "X" => Outcome::Lose, 
                "Y" => Outcome::Draw,
                "Z" => Outcome::Win,
                _ => unreachable!("Error in input"),
            };
            get_choice(opponent_choice, my_outcome) as usize + my_outcome as usize
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
        let input = r#"A Y
B X
C Z
"#;

        assert_eq!(solve(input), 12);
    }

    #[test]
    fn example_two() {
        let input = r#"A X
A X
A X
"#;

        assert_eq!(solve(input), 9);
    }
}
