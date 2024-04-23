mod task;
mod task_manager;
use std::io;

use crate::task_manager::TaskManager;


fn main(){
    let mut task_manager = TaskManager::new();

    //Adding some Dummy tasks
    task_manager.add_task("This is task 1\n".to_string());
    task_manager.add_task("This is task 2\n".to_string());
    task_manager.add_task("This is task 3\n".to_string());
    // Loop to view & process the CLI Menu

    loop {
        println!(r"Jai Shri Ram __/\__");
        println!("\nThis is a Todo CLI app");
        println!("1. To add new task, please press 1 and Enter ->");
        println!("2. To list all the existing tasks, Please press 2 and Enter ->");
        println!("3. To mark a task as done, please press 3 and Enter ->");
        println!("4. To delete an existing task, please press 4 and Enter ->");
        println!("5. To Exit, please press 5 and Enter ->");

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
            3 => {
                println!("Enter the task number you want to mark as done:");
                let mut task_id = String::new();
                io::stdin().read_line(&mut task_id).expect("Failed to readline!");
                let task_id:u32 = match task_id.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid Task number!");
                        continue;
                    }
                };
                task_manager.mark_task(task_id);
                println!("{} - Task is marked as done ", task_id)
            }
            4 => {
                println!("Enter the task number you want to delete from the list:");
                let mut task_id: String = String::new();
                io::stdin().read_line(&mut task_id).expect("Failed to readline!");
                let task_id: u32 = match task_id.trim().parse(){
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid task number!");
                        continue;
                    }
                };
                task_manager.delete_task(task_id);

            }
            5 => break,
            _ => println!("Invalid choice!\n\n")
        }
    }
}