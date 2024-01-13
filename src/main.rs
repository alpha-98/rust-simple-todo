struct Task {
    id: u32,
    description: String,
    completed: bool,
}

static mut NEXT_ID: u32 = 1;

fn add_task(description: &str) -> Task {
    unsafe {
        let id = NEXT_ID;
        NEXT_ID += 1;
        Task {
            id,
            description: description.to_string(),
            completed: false,
        }
    }
}

fn complete_task(id: usize, tasks: &mut Vec<Task>) -> Option<&Task> {
    tasks.iter_mut().find(|task| task.id == id as u32).map(|task| {
        task.completed = true;
        task as &Task
    })
}

fn list_tasks(tasks: &Vec<Task>) {
    for task in tasks {
        println!(
            "Task {}: {} ({})",
            task.id,
            task.description,
            if task.completed { "Completed" } else { "Pending" }
        );
    }
}

pub fn main() {
    let mut tasks = Vec::new();

    tasks.push(add_task("Finish homework"));
    tasks.push(add_task("Buy groceries"));
    tasks.push(add_task("Clean the house"));

    list_tasks(&tasks);

    complete_task(2, &mut tasks); // Complete task 2

    println!("\nAfter completion:");
    list_tasks(&tasks);
}
