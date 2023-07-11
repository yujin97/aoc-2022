use std::fs;

fn main() {
    let list = fs::read_to_string("src/input/d1a.txt").expect("file does not exist");
    let mut current: u32 = 0;
    let mut calories = Vec::new();
    for line in list.lines() {
        if line.trim().is_empty() {
            calories.push(current);
            current = 0;
            continue;
        }
        let val = line.trim().parse::<u32>().expect("failed to parse to 32");
        current += val;
    }

    calories.push(current);

    calories.sort();

    let mut ans = 0;

    for calorie in calories.iter().rev().take(3) {
        ans += calorie;
    }

    println!("The top three elves are carrying {} calories of snack", ans);
}
