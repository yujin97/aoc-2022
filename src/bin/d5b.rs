use std::fs;

#[derive(Debug)]
struct Stack {
    id: usize,
    cargos: Vec<char>,
}

#[derive(Debug)]
struct Instruction {
    origin: usize,
    destination: usize,
    quantity: usize,
}

impl Stack {
    fn new(id: usize, drawing: &str) -> Self {
        let mut cargos = Vec::new();
        let cargo_size: usize = 3;
        let start = (id - 1) * 4;
        let end = start + cargo_size;
        for layer in drawing.lines().rev().skip(1) {
            let cargo = &layer[start..end];
            if let Some(label) = cargo.chars().nth(1) {
                if label.is_alphabetic() {
                    cargos.push(label);
                }
            }
        }
        Self { id, cargos }
    }
}

impl From<&str> for Instruction {
    fn from(instruction: &str) -> Self {
        let mut parts = instruction.split(' ');
        let quantity = parts.nth(1).unwrap().parse::<usize>().unwrap();
        let origin = parts.nth(1).unwrap().parse::<usize>().unwrap();
        let destination = parts.nth(1).unwrap().parse::<usize>().unwrap();

        Self {
            origin,
            destination,
            quantity,
        }
    }
}

fn execute_instruction(instruction: &Instruction, stacks: &mut Vec<Stack>) {
    let &Instruction {
        origin,
        destination,
        quantity,
    } = instruction;
    let mut group = Vec::new();
    for _ in 0..quantity {
        let origin = stacks.get_mut(origin - 1).unwrap();
        let cargo = origin.cargos.pop().unwrap();
        group.push(cargo);
    }
    for _ in 0..group.len() {
        let cargo = group.pop().unwrap();
        let destination = stacks.get_mut(destination - 1).unwrap();
        destination.cargos.push(cargo);
    }
}

fn main() {
    let input = fs::read_to_string("src/input/d5.txt").expect("failed to load input");

    let drawing: String = input
        .lines()
        .take_while(|line| !line.trim().is_empty())
        .map(|line| {
            let mut line = String::from(line);
            line.push('\n');
            line
        })
        .collect();
    let mut stacks = Vec::new();
    for id in 1..=9 {
        stacks.push(Stack::new(id, &drawing));
    }

    for instruction in input
        .lines()
        .skip_while(|line| !line.trim().is_empty())
        .skip(1)
    {
        let instruction = Instruction::from(instruction);
        execute_instruction(&instruction, &mut stacks);
    }
    for stack in stacks {
        println!(
            "The cargo on the top of stack {} is {}",
            stack.id,
            stack.cargos.last().unwrap()
        );
    }
}
