use std::fs;

enum Gesture {
    Rock,
    Paper,
    Scissors,
}

enum Status {
    Win,
    Lose,
    Draw,
}

impl From<char> for Gesture {
    fn from(c: char) -> Gesture {
        match c {
            'A' => Gesture::Rock,
            'B' => Gesture::Paper,
            'C' => Gesture::Scissors,
            _ => panic!("failed to convert {} to Gesture", c),
        }
    }
}

impl Gesture {
    fn score(&self) -> u8 {
        match self {
            Gesture::Rock => 1,
            Gesture::Paper => 2,
            Gesture::Scissors => 3,
        }
    }
}

impl From<char> for Status {
    fn from(c: char) -> Self {
        match c {
            'X' => Status::Lose,
            'Y' => Status::Draw,
            'Z' => Status::Win,
            _ => panic!("failed to convert {} to Status", c),
        }
    }
}

fn get_self_from_result(opponent: &Gesture, result: &Status) -> Gesture {
    match opponent {
        Gesture::Scissors => match result {
            Status::Draw => Gesture::Scissors,
            Status::Lose => Gesture::Paper,
            Status::Win => Gesture::Rock,
        },
        Gesture::Rock => match result {
            Status::Win => Gesture::Paper,
            Status::Draw => Gesture::Rock,
            Status::Lose => Gesture::Scissors,
        },
        Gesture::Paper => match result {
            Status::Lose => Gesture::Rock,
            Status::Win => Gesture::Scissors,
            Status::Draw => Gesture::Paper,
        },
    }
}

fn calculate_score(opponent: &Gesture, result: &Status) -> u8 {
    match result {
        Status::Win => get_self_from_result(opponent, result).score() + 6,
        Status::Lose => get_self_from_result(opponent, result).score(),
        Status::Draw => get_self_from_result(opponent, result).score() + 3,
    }
}

fn process_line(line: &str) -> u8 {
    let opponent = Gesture::from(line.chars().nth(0).expect("failed to extract char"));
    let result = Status::from(line.chars().nth(2).expect("failed to extract char"));

    calculate_score(&opponent, &result)
}

fn main() {
    let guide = fs::read_to_string("src/input/d2.txt").expect("failed to read file");

    let mut total_score = 0;

    for line in guide.lines() {
        total_score += process_line(line) as u32;
    }

    println!("The total score is {total_score}");
}

#[cfg(test)]
mod tests {
    use crate::process_line;

    #[test]
    fn rock_draw() {
        let input = "A Y";
        let score = process_line(input);
        assert_eq!(score, 4);
    }

    #[test]
    fn rock_lose() {
        let input = "B X";
        let score = process_line(input);
        assert_eq!(score, 1);
    }

    #[test]
    fn scissors_win() {
        let input = "C Z";
        let score = process_line(input);
        assert_eq!(score, 7);
    }
}
