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

    let mut rucksacks = rucksacks.lines();

    let mut rucksack = rucksacks.next();

    while rucksack.is_some() {
        let first_elf = rucksack.expect("invalid rucksack");

        let mut first_rucksack = HashMap::new();
        for item in first_elf.chars() {
            first_rucksack.insert(item, 1);
        }

        let second_elf = rucksacks.next().expect("invalid rucksack");
        let mut second_rucksack = HashMap::new();

        for item in second_elf.chars() {
            second_rucksack.insert(item, 1);
        }

        let third_elf = rucksacks.next().expect("invalid rucksack");
        let mut third_rucksack = HashMap::new();

        for item in third_elf.chars() {
            third_rucksack.insert(item, 1);
        }

        let mut summary = first_rucksack;

        for item in second_rucksack.keys() {
            if summary.contains_key(item) {
                let count = summary.get_mut(item).unwrap();
                *count = 2;
            }
        }

        for item in third_rucksack.keys() {
            if summary.contains_key(item) {
                let count = summary.get_mut(item).unwrap();
                if count == &2 {
                    *count = 3;
                }
            }
        }

        for (item, count) in summary.iter() {
            if count == &3 {
                let priority = priority_map.get(&item).expect("invalid character");
                total += *priority as i32;
            }
        }

        rucksack = rucksacks.next();
    }

    println!("sum of the priorities: {}", total);
}
