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

fn get_outcome(my_choice: RPS, opponent_choice: RPS) -> Outcome {
    match (my_choice, opponent_choice) {
        (RPS::Rock, RPS::Rock) => Outcome::Draw,
        (RPS::Rock, RPS::Paper) => Outcome::Lose,
        (RPS::Rock, RPS::Scissors) => Outcome::Win,
        (RPS::Paper, RPS::Rock) => Outcome::Win,
        (RPS::Paper, RPS::Paper) => Outcome::Draw,
        (RPS::Paper, RPS::Scissors) => Outcome::Lose,
        (RPS::Scissors, RPS::Rock) => Outcome::Lose,
        (RPS::Scissors, RPS::Paper) => Outcome::Win,
        (RPS::Scissors, RPS::Scissors) => Outcome::Draw,
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

            let my_choice = match right {
                "X" => RPS::Rock, 
                "Y" => RPS::Paper,
                "Z" => RPS::Scissors,
                _ => unreachable!("Error in input"),
            };
            my_choice as usize + get_outcome(my_choice, opponent_choice) as usize
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

        assert_eq!(solve(input), 15);
    }

    #[test]
    fn example_two() {
        let input = r#"A X
A X
A X
"#;

        assert_eq!(solve(input), 12);
    }
}
