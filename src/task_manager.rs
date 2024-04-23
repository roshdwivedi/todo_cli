use crate::task::Task;

pub struct TaskManager {
    tasks: Vec<Task>
}

impl TaskManager {
    pub fn new() -> Self {
        TaskManager { tasks: Vec::new() }
    }
    pub fn add_task(&mut self, description: String) {
        let id = (self.tasks.len() + 1) as u32;
        let task = Task {id, description, completed: false};
        self.tasks.push(task);
    }
    pub fn tasks_list(&self) {
        for task in &self.tasks {
            println!("{} - {}: {}", task.id, task.description, task.completed)
        }
    }
}
    