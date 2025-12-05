use std::collections::HashMap;
use std::fmt;
use std::io::{self, BufRead};
use std::fs::File;
use std::io::Write;
use std::fs;

#[derive(Debug)]
pub struct TaskItem {
    id: u32,
    description: String,
    priority: String,
    state: State,
}

impl TaskItem {
    pub fn new(id: u32, description: String, priority: String) -> TaskItem {
        TaskItem {
            id,
            description,
            priority,
            state: State::Todo,
        }
    }

    pub fn state_done(&mut self) {
        self.state = State::Done;
    }
}


impl fmt::Display for TaskItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[#{:05}] <{:?}> {} -{:?}",
            self.id, self.priority, self.description, self.state
        )
    }
}


pub struct Task {
    tasks: HashMap<u32, TaskItem>,
}

impl Task {
    pub fn new() -> Self {
        Self {
            tasks: HashMap::new(),
        }
    }

    pub fn add_task(&mut self, id: u32, tasklist: TaskItem) {
        self.tasks.insert(id, tasklist);
    }

    pub fn delete_task(&mut self, id: u32) -> Option<TaskItem> {
        self.tasks.remove(&id)
    }

    pub fn find_task(&self, id: u32) -> Option<&TaskItem> {
        self.tasks.get(&id)
    }

    pub fn done_task(&mut self, id: u32) -> bool {
        if let task_target = self.tasks.get_mut(&id) {
            task_target.unwrap().state = State::Done;
            return true
        }
        false
    }
}

#[derive(Debug)]
pub enum State {
    Todo,
    Done,
}


pub fn input_file(task_: &Task, filename: &String) {
    let mut contents = String::new();
    for (_, taskitem) in task_.tasks.iter() {
        contents.push_str(&format!(
        "[#{:05}] <{}> {} -{:?}\n",
        taskitem.id,
        taskitem.priority,
        taskitem.description,
        taskitem.state
    ));
    }
    fs::write(filename, contents)
        .expect("Failed to write to todo_list.txt");
}

pub fn input_task(input: String, task: &mut Task, file: &mut File) -> Option<()> {
    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.len() > 3 && parts[0].eq_ignore_ascii_case("add") {
        let id = parts[1].trim();
        let priority_read = parts[2].trim();
        let description_read = parts[3..].join(" ");
        if let Ok(id) = id.parse::<u32>() {
            let task_item = TaskItem::new(id, description_read.to_string(), priority_read.to_string());
            task.add_task(id, task_item);
            println!("Task added successfully.");
        } else {
            println!("Invalid command format. Use: add <id> <priority> <description>");
        }
    } else {
        println!("Invalid command format.  Use: add <id> <priority> <description>");
    }
    Some(())
}

pub fn read_file_task(line: String, task: &mut Task) -> Option<()> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    let id = parts[0].trim_start_matches("[#").trim_end_matches("]");
    let priority_read = parts[1].trim_start_matches("<").trim_end_matches(">");
    let description_read = parts[2..parts.len()-1].join(" ");
    let state_read = parts[parts.len()-1].trim();
    if let Ok(id) = id.parse::<u32>() {
        let mut task_item = TaskItem::new(id, description_read.to_string(), priority_read.to_string());
        task_item.state = if state_read == "-Todo" {
            State::Todo
        } else {
            State::Done
        };
        task.add_task(id, task_item);
    } else {
        println!("File data error. Invalid ID.");
    }
    Some(())
}



pub fn help_input() {
    println!("Available commands:");
    println!("add <id> <priority> <description> - Add a New Task");
    println!("delete <id> - Delete a Task by ID");
    println!("check <id> - Check a Task by ID");
    println!("done <id> - Mark a Task as Done by ID");
    println!("quit - Exit the Application");
}


pub fn delete(id_str: &str, task: &mut Task) {
    if let Ok(id) = id_str.parse::<u32>() {
        let deleted_task = task.delete_task(id);
        if let Some(task) = deleted_task {
            println!("Delete task:\n{}", task);
        } else {
            println!("Task with ID {} not found.", id);
        }
    } else {
        println!("Invalid ID format.");
    }
}

pub fn check(id_str: &str, task:&mut Task) {
    if let Ok(id) = id_str.parse::<u32>() {
        let found_task = task.find_task(id);
        if let Some(task) = found_task {
            println!("Found task:\n{}", task);
        } else {
            println!("Task with ID {} not found.", id);
        }
    } else {
        println!("Invalid ID format.");
    }
}

pub fn done(id_str: &str, task: &mut Task) {
    if let Ok(id) = id_str.parse::<u32>() {
        if task.done_task(id) {
            println!("Task with ID {} marked as done.", id);
        } else {
            println!("Task with ID {} not found.", id);
        }
    } else {
        println!("Invalid ID format.");
    }
}