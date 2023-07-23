use std::collections::hash_map::HashMap;
use std::fs;

fn main() {
    let rucksacks = fs::read_to_string("src/input/d3.txt").expect("failed to read file");
    let mut total = 0;

    let mut priority_map = HashMap::new();

    for (idx, item) in ('a'..='z').enumerate() {
        priority_map.insert(item, 1 + idx);
    }

    for (idx, item) in ('A'..='Z').enumerate() {
        priority_map.insert(item, 27 + idx);
    }

    for rucksack in rucksacks.lines() {
        let mut map = HashMap::new();

        let first_half = rucksack.chars().take(rucksack.len() / 2);
        let second_half = rucksack.chars().skip(rucksack.len() / 2);

        for item in first_half {
            map.insert(item, true);
        }

        for item in second_half {
            if map.contains_key(&item) {
                // get score and add to total
                let priority = priority_map.get(&item).expect("invalid character");
                total += *priority as i32;
                break;
            }
        }
    }

    println!("sum of the priorities: {}", total);
}
