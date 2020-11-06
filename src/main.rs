use std::io;
use std::fmt;

#[derive(Debug)]
struct Task {
    id: u32,
    completed: bool,
    text: String,
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let check = if self.completed {"X"} else {" "};
        write!(f, "[{}] Task {}: {}", check, self.id, self.text)
    }
}

fn new_task(id: u32, text: String) -> Task {
    Task {
        id,
        completed: false,
        text,
    }
}

fn main() {
    let mut task_list: Vec<Task> = Vec::new();
    for id in 0..5 {
        println!("Please enter the task to be completed.");
        let mut text = String::new();
        io::stdin()
            .read_line(&mut text)
            .expect("Failed to read line.");
        task_list.push(new_task(id, text));
    }

    println!("Contents of task_list:");
    for t in task_list.iter() {
        println!("> {}", t);
    }
}
