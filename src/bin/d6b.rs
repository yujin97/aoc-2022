use std::collections::HashMap;
use std::fs;

fn find_first_message_marker(signal: &str) -> usize {
    for (idx, window) in signal.as_bytes().windows(14).enumerate() {
        let mut count = HashMap::new();
        for c in window {
            if !count.contains_key(c) {
                count.insert(c, true);
            }
        }
        if count.len() == 14 {
            return idx + 14;
        }
    }
    return signal.len();
}

fn main() {
    let message = fs::read_to_string("src/input/d6.txt").expect("failed to read file.");

    let first_marker = find_first_message_marker(&message);

    println!(
        "{} characters need to be processed before the first start-of-message marker is detected.",
        first_marker
    );
}
