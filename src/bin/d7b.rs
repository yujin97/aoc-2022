use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::rc::{Rc, Weak};

struct Directory {
    parent: Option<Weak<RefCell<Directory>>>,
    children: HashMap<String, Rc<RefCell<Directory>>>,
    file_sizes: Vec<usize>,
}

impl Directory {
    fn new(parent: Option<Weak<RefCell<Directory>>>) -> Self {
        Directory {
            parent,
            children: HashMap::new(),
            file_sizes: Vec::new(),
        }
    }

    fn add_file(&mut self, file: File) {
        self.file_sizes.push(file.0);
    }

    fn add_new_directory(&mut self, dir_name: String, new_dir: Rc<RefCell<Directory>>) {
        self.children.insert(dir_name, new_dir);
    }

    fn get_child(&mut self, dir_name: &str) -> Rc<RefCell<Directory>> {
        Rc::clone(&self.children.get_mut(dir_name).unwrap())
    }

    fn get_parent(&self) -> Rc<RefCell<Directory>> {
        Weak::clone(&self.parent.as_ref().unwrap())
            .upgrade()
            .unwrap()
    }

    fn get_size(&self) -> usize {
        let file_size = self
            .file_sizes
            .iter()
            .map(|size| size.to_owned())
            .reduce(|sum, elem| sum + elem)
            .unwrap_or(0);

        let mut dir_size = 0usize;

        for (_, dir) in self.children.iter() {
            dir_size += dir.borrow().get_size();
        }

        file_size + dir_size
    }
}

#[derive(Debug)]
struct File(usize);

impl File {
    fn new(size: usize) -> Self {
        File(size)
    }
}

#[derive(Debug)]
enum TerminalOutput {
    CD(String),
    LS,
    FileOutput(File),
    DirOutput(String),
}

impl From<&str> for TerminalOutput {
    fn from(output: &str) -> Self {
        let mut segments = output.split(' ');
        let first = segments.next().unwrap();
        match first {
            "$" => {
                let cmd = segments.next().unwrap();
                match cmd {
                    "cd" => {
                        let dir = segments.next().unwrap();
                        let dir = String::from(dir);
                        Self::CD(dir)
                    }
                    "ls" => Self::LS,
                    _ => panic!("invalid user command"),
                }
            }
            "dir" => {
                let dir = segments.next().unwrap();
                let dir = String::from(dir);
                Self::DirOutput(dir)
            }
            size => {
                let size = size.parse::<usize>().unwrap();
                let file = File::new(size);
                Self::FileOutput(file)
            }
        }
    }
}

fn get_ans_by_dfs(dir: Rc<RefCell<Directory>>, target: u32) -> Option<u32> {
    let total_space = 70000000;
    let mut min = total_space;
    for (_, child) in dir.borrow().children.iter() {
        let child = Rc::clone(child);
        let child_min = get_ans_by_dfs(child, target);
        if let Some(child_min) = child_min {
            if child_min < min && child_min > target {
                min = child_min;
            }
        }
    }

    let size = dir.borrow().get_size() as u32;

    if size < min && size > target {
        return Some(size);
    }

    if min < total_space {
        return Some(min);
    } else {
        return None;
    }
}

fn main() {
    use TerminalOutput::*;
    let output = fs::read_to_string("src/input/d7.txt").expect("failed to read input");

    let head = Rc::new(RefCell::new(Directory::new(None)));

    let mut current = Rc::clone(&head);

    for line in output.lines() {
        let output = TerminalOutput::from(line);

        match output {
            CD(dir_name) if dir_name == "/" => {}
            CD(dir_name) if dir_name == ".." => {
                let next_current = current.borrow().get_parent();
                current = next_current;
            }
            CD(dir_name) => {
                let new_dir = Rc::new(RefCell::new(Directory::new(Some(Rc::downgrade(&current)))));
                current
                    .borrow_mut()
                    .add_new_directory(dir_name, Rc::clone(&new_dir));
                current = new_dir;
            }
            LS => {}
            FileOutput(file) => {
                current.borrow_mut().add_file(file);
            }
            DirOutput(dir_name) => {
                let new_dir = Rc::new(RefCell::new(Directory::new(Some(Rc::downgrade(&current)))));
                current.borrow_mut().add_new_directory(dir_name, new_dir);
            }
        }
    }

    let total_space = 70000000u32;

    let space_needed = 30000000u32;

    let current_availabe = total_space - head.borrow().get_size() as u32;

    let target = space_needed - current_availabe;

    let ans = get_ans_by_dfs(Rc::clone(&head), target);

    println!("Total size of the target directory is {}", ans.unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn one_file() {
        let parent = Rc::new(RefCell::new(Directory::new(None)));
        let file_1 = File(3083);
        parent.borrow_mut().add_file(file_1);

        let new_dir = Rc::new(RefCell::new(Directory::new(Some(Rc::downgrade(&parent)))));
        parent
            .borrow_mut()
            .add_new_directory(String::from("first"), new_dir);
        let file_2 = File(1000);

        let child = parent.borrow_mut().get_child("first");
        child.borrow_mut().add_file(file_2);
        let size = parent.borrow().get_size();

        assert_eq!(size, 4083)
    }
}
