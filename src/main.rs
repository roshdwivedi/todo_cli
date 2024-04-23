mod task;
mod task_manager;
use std::io;

use task_manager::TaskManager;

fn main(){
    let mut task_manager = TaskManager::new();

    loop {
        println!(r"Jai Shri Ram __/\__");
        println!("This is a Todo CLI app");
        println!("1. To add new task, please press 1 and Enter ->");
        println!("2. To list all the existing tasks, Please press 2 and Enter ->");
        println!("2. To Exit, please press 3 and Enter ->");

        let mut choice  = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line, Please enter a u32 number");
        let choice:u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match choice {
            1 => {
                println!("Entere the description of the task you want to add!");
                let mut description = String::new();
                io::stdin().read_line(&mut description).expect("Failed to read line!");
                task_manager.add_task(description.trim().to_string());
            },
            2 => task_manager.tasks_list(),
            3 => break,
            _ => println!("Invalid choice!")
        }
    }
}