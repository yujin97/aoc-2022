use std::fs;

fn main() {
    let list = fs::read_to_string("src/input/d1a.txt").expect("file does not exist");
    let mut max: u32 = 0;
    let mut current: u32 = 0;
    for line in list.lines() {
        if line.trim().is_empty() {
            if current > max {
                max = current;
            }
            current = 0;
            continue;
        }
        let val = line.trim().parse::<u32>().expect("failed to parse to 32");
        current += val;
    }

    if current > max {
        max = current;
    }

    println!("The most calories carried by the elfs is {}", max);
}
