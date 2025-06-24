use std::io;
fn main() {
    println!("Welcome to the task processing system!");
    println!("Pleae type your tasks");
    let tasks = ask_user();

    let high_priority_tasks = process_tasks(&tasks);
    println!("There are {} high priority tasks", high_priority_tasks);
}

fn process_tasks(tasks: &[(String, String)]) -> i16 {
    let mut high_priority_tasks = 0;
    'outer: loop {
        for (task_name, priority) in tasks.iter() {
            println!("Processing task: {}", task_name);
            if priority == &"high" {
                high_priority_tasks += 1;
            }
            if high_priority_tasks == 3 {
                println!("Total high priority tasks found: 3");
                break 'outer;
            }
        }
        if high_priority_tasks < 3 {
            println!("No more high priority tasks found.");
            break 'outer;
        }
    }
    return high_priority_tasks;
}

fn ask_user() -> Vec<(String, String)> {
    let mut vec_of_tasks: Vec<(String, String)> = vec![];
    println!("Please enter the number of tasks you want to add:");
    let mut number_of_tasks = String::new();

    io::stdin()
        .read_line(&mut number_of_tasks)
        .expect("failed to readline");

    match number_of_tasks.trim().parse::<i16>() {
        Ok(number) => {
            for _ in 0..number {
                println!("Enter task name:");
                let mut task_name = String::new();
                io::stdin()
                    .read_line(&mut task_name)
                    .expect("failed to readline");
                println!("Enter task priority (low, medium, high):");
                let mut task_priority = String::new();
                io::stdin()
                    .read_line(&mut task_priority)
                    .expect("failed to readline");

                let task = (
                    task_name.trim().to_string(),
                    task_priority.trim().to_string(),
                );
                vec_of_tasks.push(task);
            }
        }
        Err(e) => panic!("Error: {}", e), // Will throw error with `panic!` macro.
    };
    return vec_of_tasks;
}
