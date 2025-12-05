use std::collections::HashMap;
use std::fmt;
use std::io::{self, BufRead};
use std::io::Write;
use std::fs::OpenOptions;
use std::fs::File;
use TodoListCLI::*;

fn main() {
    let stdin = io::stdin();
    let mut task = Task::new();
    let filename = String::from("todo_list.txt");

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(true)
        .open(&filename)
        .expect("Failed to open todo_list.txt");

    let reader: Vec<String> = io::BufReader::new(&file)
        .lines()
        .filter_map(Result::ok)
        .collect();
    println!("Welcome to Todo List CLI!");
    println!("Type 'help' to see available commands.");
    println!("-----------------------------------");
    if reader.is_empty() {
        println!("No existing tasks found.");
    } else {
        for line in reader {
            read_file_task(line, &mut task);
        }
    }
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if stdin.lock().read_line(&mut input).is_err() {
            break;
        }
        let input = input.trim();

        if input.is_empty() {
            continue;
        }else if input.eq_ignore_ascii_case("help") {
            help_input();
            continue;
        }

        if input.eq_ignore_ascii_case("quit") { 
            println!("Exiting Todo List CLI. Goodbye!");
            break;
        }

        if input.to_lowercase().starts_with("add ") {
            input_task(input.to_string(), &mut task, &mut file);
        }else if input.to_lowercase().starts_with("delete ") {
            let id_str = &input[7..].trim();
            delete(id_str, &mut task);
        } else if input.to_lowercase().starts_with("check ") {
            let id_str = &input[6..].trim();
            check(id_str, &mut task);
        } else if input.to_lowercase().starts_with("done ") {
            let id_str = &input[5..].trim();
            done(id_str, &mut task);
        }else if input.to_lowercase().starts_with("save") {
            input_file(&task, &filename);
            println!("Tasks saved to file.");
        } else {
            println!("Unknown command. Type 'help' to see available commands.");
            continue;
        }
        }

        println!("-----------------------------------");
    }



        
    
    

