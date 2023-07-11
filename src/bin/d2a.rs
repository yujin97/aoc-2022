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
            'A' | 'X' => Gesture::Rock,
            'B' | 'Y' => Gesture::Paper,
            'C' | 'Z' => Gesture::Scissors,
            _ => panic!("failed to convert {} to Gesture", c),
        }
    }
}

impl Gesture {
    fn judge(&self, opponent: &Self) -> Status {
        match self {
            Gesture::Scissors => match opponent {
                Gesture::Scissors => Status::Draw,
                Gesture::Rock => Status::Lose,
                Gesture::Paper => Status::Win,
            },
            Gesture::Rock => match opponent {
                Gesture::Scissors => Status::Win,
                Gesture::Rock => Status::Draw,
                Gesture::Paper => Status::Lose,
            },
            Gesture::Paper => match opponent {
                Gesture::Scissors => Status::Lose,
                Gesture::Rock => Status::Win,
                Gesture::Paper => Status::Draw,
            },
        }
    }

    fn score(&self) -> u8 {
        match self {
            Gesture::Rock => 1,
            Gesture::Paper => 2,
            Gesture::Scissors => 3,
        }
    }
}

fn calculate_score(opponent: &Gesture, myself: &Gesture) -> u8 {
    let result = myself.judge(opponent);
    match result {
        Status::Win => myself.score() + 6,
        Status::Lose => myself.score(),
        Status::Draw => myself.score() + 3,
    }
}

fn main() {
    let guide = fs::read_to_string("src/input/d2.txt").expect("failed to read file");

    let mut total_score = 0;

    for line in guide.lines() {
        let opponent = Gesture::from(line.chars().nth(0).expect("failed to extract char"));
        let myself = Gesture::from(line.chars().nth(2).expect("failed to extract char"));

        total_score += calculate_score(&opponent, &myself) as u32;
    }

    println!("The total score is {total_score}");
}
