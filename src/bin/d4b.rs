use std::fs;

struct Assignment {
    lower: u16,
    upper: u16,
}

impl From<&str> for Assignment {
    fn from(range: &str) -> Self {
        let parts: Vec<&str> = range.split("-").collect();
        let lower = parts[0].parse::<u16>().unwrap();
        let upper = parts[1].parse::<u16>().unwrap();
        Self { lower, upper }
    }
}

impl Assignment {
    fn contains(&self, range: &Assignment) -> bool {
        self.upper >= range.upper && self.lower <= range.lower
    }

    fn overlaps(&self, range: &Assignment) -> bool {
        match range {
            Assignment { lower, upper }
                if self.lower <= *lower && self.upper >= *lower && self.upper <= *upper =>
            {
                true
            }
            Assignment { lower, upper }
                if self.lower >= *lower && self.lower <= *upper && self.upper >= *upper =>
            {
                true
            }
            range if self.contains(range) || range.contains(self) => true,
            _ => false,
        }
    }
}

fn main() {
    let pairs = fs::read_to_string("src/input/d4.txt").expect("failed to read input file.");

    let mut total = 0;

    for pair in pairs.lines() {
        let ranges: Vec<&str> = pair.split(",").collect();
        let range1 = ranges[0];
        let range2 = ranges[1];

        let assignment1 = Assignment::from(range1);
        let assignment2 = Assignment::from(range2);

        if assignment1.overlaps(&assignment2) {
            total += 1;
        }
    }

    println!(
        "total number of assignment pairs that ranges overlap: {}",
        total
    );
}
