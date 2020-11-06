use std::fmt;
use std::io;

struct Task {
    id: u32,
    completed: bool,
    text: String,
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let check = if self.completed { "X" } else { " " };
        write!(f, "[{}] Task {}: {}", check, self.id, self.text)
    }
}

fn create_task(id: u32, text: String) -> Task {
    Task {
        id,
        completed: false,
        text,
    }
}

fn list_all_tasks(task_list: &std::vec::Vec<Task>) {
    println!("\nCurrent Task List:");
    for t in task_list {
        println!("> {}", t);
    }
}

fn complete_task(task_list: &mut std::vec::Vec<Task>) {
    list_all_tasks(&task_list);
    let id: u32 = {
        loop {
            println!("\nWhich task should be completed?");
            let mut id = String::new();
            io::stdin().read_line(&mut id).expect("Failed to read line");
            match id.trim().parse() {
                Ok(num) => break num,
                Err(_) => continue,
            };
        }
    };
    for (index, task) in task_list.iter().enumerate() {
        if task.id == id {
            println!("Marked task {} as complete.", id);
            task_list[index].completed = true;
            break;
        }
    }
}

fn main() {
    let mut task_list: Vec<Task> = Vec::new();
    let mut id: u32 = 0;

    loop {
        println!("What would you like to do?\n[n] New Task\n[l] List Tasks\n[c] Mark Task as Complete\n[q] Quit");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");
        match input.trim_end() {
            "n" => {
                println!("Please enter the task to be completed.");
                let mut text = String::new();
                io::stdin()
                    .read_line(&mut text)
                    .expect("Failed to read line.");
                id += 1;
                task_list.push(create_task(id, text));
            }
            "l" => list_all_tasks(&task_list),
            "c" => complete_task(&mut task_list),
            "q" => break,
            _ => {
                println!("Invalid input.");
                continue;
            }
        }
    }
}
