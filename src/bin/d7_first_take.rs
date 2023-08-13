// first attempt of the challenge
// does not work

use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::rc::Rc;

#[derive(Debug)]
enum FileTree {
    Dir {
        full_path: String,
        parent: Rc<RefCell<FileTree>>,
        children: HashMap<String, Rc<RefCell<FileTree>>>,
    },
    File {
        parent: Rc<RefCell<FileTree>>,
        full_path: String,
        size: usize,
    },
    Nil,
}

impl FileTree {
    fn new_file(
        file_name: String,
        size: usize,
        parent: Rc<RefCell<FileTree>>,
        parent_full_path: String,
    ) -> FileTree {
        let full_path = format!("{}/{}", parent_full_path, file_name);

        let parent_rc = Rc::clone(&parent);

        FileTree::File {
            parent: parent_rc,
            full_path,
            size,
        }
    }

    fn new_dir(
        dir_name: String,
        parent: Rc<RefCell<FileTree>>,
        parent_full_path: String,
    ) -> FileTree {
        let full_path = format!("{}/{}", parent_full_path, dir_name);

        let parent_rc = Rc::clone(&parent);
        let children = HashMap::new();

        FileTree::Dir {
            full_path,
            parent: parent_rc,
            children,
        }
    }

    fn get_full_path(&self) -> String {
        match self {
            FileTree::Nil => String::from('/'),
            FileTree::Dir { full_path, .. } => full_path.to_owned(),
            FileTree::File { full_path, .. } => full_path.to_owned(),
        }
    }

    fn get_child(&self, dir_name: String) -> Rc<RefCell<FileTree>> {
        match self {
            FileTree::Dir { children, .. } => Rc::clone(children.get(&dir_name).unwrap()),
            _ => panic!("trying to get child from non-directory node"),
        }
    }

    fn get_parent(&self) -> Rc<RefCell<FileTree>> {
        match self {
            FileTree::Dir { parent, .. } => Rc::clone(&parent),
            _ => panic!("trying to get parent from non-directory node"),
        }
    }

    fn add_child_file(
        &mut self,
        file_name: String,
        size: usize,
        parent: Rc<RefCell<FileTree>>,
        parent_full_path: String,
    ) {
        match self {
            FileTree::Dir { children, .. } => {
                let new_file = FileTree::new_file(file_name, size, parent, parent_full_path);
                children.insert(new_file.get_full_path(), Rc::new(RefCell::new(new_file)));
            }
            _ => panic!("trying to add child to non-directory node"),
        }
    }

    fn add_child_dir(
        &mut self,
        dir_name: String,
        parent: Rc<RefCell<FileTree>>,
        parent_full_path: String,
    ) {
        match self {
            FileTree::Dir { children, .. } => {
                let new_dir = FileTree::new_dir(dir_name, parent, parent_full_path);
                children.insert(new_dir.get_full_path(), Rc::new(RefCell::new(new_dir)));
            }
            _ => panic!("trying to add child to non-directory node"),
        }
    }
}

#[derive(Debug)]
enum TerminalOutput {
    CD(String),
    LS,
    FileOutput(String, usize),
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
                let file_name = segments.next().unwrap();
                Self::FileOutput(String::from(file_name), size)
            }
        }
    }
}

fn main() {
    use TerminalOutput::*;
    let output = fs::read_to_string("src/input/d7_test.txt").expect("failed to read input");

    let mut head = Rc::new(RefCell::new(FileTree::Dir {
        full_path: String::from("/"),
        parent: Rc::new(RefCell::new(FileTree::Nil)),
        children: HashMap::new(),
    }));

    let origin = Rc::clone(&head);

    for line in output.lines() {
        let output = TerminalOutput::from(line);
        match output {
            CD(dir) if dir == "/" => {}
            CD(dir) if dir == ".." => {
                let parent = (*head).borrow().get_parent();
                head = parent;
            }
            CD(dir) => {
                let dir_name = format!("{}/{}", (*head).borrow().get_full_path(), dir);
                let target = (*head).borrow().get_child(dir_name);
                head = target;
            }
            FileOutput(file_name, size) => {
                let parent_full_path = (*head).borrow().get_full_path();
                (*head).borrow_mut().add_child_file(
                    file_name,
                    size,
                    Rc::clone(&head),
                    parent_full_path,
                );
            }
            DirOutput(dir_name) => {
                let parent_full_path = (*head).borrow().get_full_path();
                (*head)
                    .borrow_mut()
                    .add_child_dir(dir_name, Rc::clone(&head), parent_full_path);
            }
            LS => {}
        };
    }
}
