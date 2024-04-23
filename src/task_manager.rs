use crate::task::Task;
use crate::task::Status;

pub struct TaskManager {
    tasks: Vec<Task>
}

impl TaskManager {
    pub fn new() -> Self {
        TaskManager { tasks: Vec::new() }
    }
    pub fn add_task(&mut self, description: String) {
        let id = (self.tasks.len() + 1) as u32;
        let task = Task {id, description, status: Status::Pending};
        self.tasks.push(task);
    }
    pub fn tasks_list(&self) {
        for task in &self.tasks {
            println!("\n\n{} - {} Status: {}\n\n", task.id, task.description, task.status)
        }
    }
    pub fn mark_task(&mut self, task_id: u32){
        for task in &mut self.tasks {
            if task.id == task_id {
                task.status = Status::Completed;
                break;
            }
        }
    }
}
    