use futures::future::join_all;
use rand::prelude::*;
use std::thread::sleep;
use std::time::Duration;
use tokio::task::spawn;

async fn process_task(task_name: String) -> Result<String, String> {
    let mut rng = rand::rng();
    let random_number = rng.random_range(50..500);
    sleep(Duration::from_millis(random_number));
    if task_name.starts_with("fail_") {
        Err(format!("Failed to process: {}", task_name))
    } else {
        Ok(format!("Successfully processed: {}", task_name))
    }
}
async fn process_all_tasks(task_names: Vec<String>) -> Vec<Result<String, String>> {
    let mut results = Vec::new();
    let tasks = task_names.clone();
    for task in tasks {
        let handler = spawn(async move { process_task(task).await });
        results.push(handler);
    }
    let handles = join_all(results).await;
    handles
        .into_iter()
        .map(|handle| handle.unwrap_or_else(|e| Err(format!("Task Panicked: {}", e))))
        .collect()
}
#[tokio::main]
async fn main() {
    let tasks = vec![
        "download_file_A".to_string(),
        "process_data_B".to_string(),
        "fail_upload_report_C".to_string(),
        "analyze_logs_D".to_string(),
        "fail_connect_to_db_E".to_string(),
    ];
    let results = process_all_tasks(tasks).await;
    for result in results {
        match result {
            Ok(msg) => println!("{}", msg),
            Err(err) => println!("Error: {}", err),
        }
    }
}
