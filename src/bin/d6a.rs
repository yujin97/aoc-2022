use std::collections::HashMap;
use std::fs;

fn find_first_marker(signal: &str) -> usize {
    for (idx, window) in signal.as_bytes().windows(4).enumerate() {
        let mut count = HashMap::new();
        for c in window {
            if !count.contains_key(c) {
                count.insert(c, true);
            }
        }
        if count.len() == 4 {
            return idx + 4;
        }
    }
    return signal.len();
}

fn main() {
    let message = fs::read_to_string("src/input/d6.txt").expect("failed to read file.");

    let first_marker = find_first_marker(&message);

    println!(
        "{} characters need to be processed before the first start-of-packet marker is detected.",
        first_marker
    );
}


