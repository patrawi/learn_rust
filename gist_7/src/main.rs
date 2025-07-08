use std::collections::HashMap;
use std::io::{self, Write};

fn get_command() -> String {
    print!("Enter command:");
    let _ = io::stdout().flush();
    let mut usr_input = String::new();
    io::stdin()
        .read_line(&mut usr_input)
        .expect("Failed to Readline");
    usr_input
}
fn main() {
    let mut inventory: HashMap<String, u32> = HashMap::new();
    loop {
        let command = get_command();

        if command.trim() == "exit" {
            println!("Exiiting inventory tracker");
            break;
        }
        let command_set = command.split_whitespace().collect::<Vec<&str>>();
        match command_set[0].trim() {
            "add" => {
                if command_set.len() != 3 {
                    println!("Usage need 3 arguments");
                    continue;
                }
                let item_name = command_set[1].to_string();
                let quantity_str = &command_set[2];
                let quantity_to_add = match quantity_str.parse::<u32>() {
                    Ok(val) => val,
                    Err(_) => {
                        println!("Invalid quantity");
                        continue;
                    }
                };
                let current_stock_ref = inventory.entry(item_name.clone()).or_insert(0);
                *current_stock_ref += quantity_to_add;
                println!(
                    "Added {} {}. Current stock: {}",
                    quantity_to_add, item_name, current_stock_ref
                );
            }
            "remove" => {
                if command_set.len() != 3 {
                    println!("Usage need 3 arguments");
                    continue;
                }
                let item_name = command_set[1].to_string();
                let quantity_str = &command_set[2];
                let quantity_to_remove = match quantity_str.parse::<u32>() {
                    Ok(val) => val,
                    Err(_) => {
                        println!("Invalid quantity");
                        continue;
                    }
                };
                if let Some(current_stock) = inventory.get_mut(&item_name) {
                    match current_stock.checked_sub(quantity_to_remove) {
                        Some(remaining_stock) => {
                            if remaining_stock == 0 {
                                inventory.remove(&item_name);
                                println!(
                                    "Removed {} {}. Item removed from inventory.",
                                    quantity_to_remove, item_name
                                );
                            } else {
                                *current_stock = remaining_stock;
                                println!(
                                    "Removed {} {}. Current stock: {}.",
                                    quantity_to_remove, item_name, remaining_stock
                                );
                            }
                        }
                        None => {
                            println!(
                                "Error: Not enough {} to remove. Current stock: {}.",
                                item_name, *current_stock
                            );
                        }
                    }
                }
            }
            "check" => {
                if command_set.len() != 2 {
                    println!("Usage need 2 arguments");
                    continue;
                }
                let item_name = command_set[1].to_string();
                match inventory.get(&item_name) {
                    Some(val) => println!("{}: {}", item_name, val),
                    None => {
                        println!("Error: {} not found in inventory.", item_name);
                    }
                }
            }
            "list" => {
                if command_set.len() != 1 {
                    println!("Usage need 1 arguments");
                    continue;
                }
                println!("Current Inventory:");
                for (key, value) in inventory.iter() {
                    println!("{}: {}", key, value);
                }
            }
            _ => {
                println!("Invalid command");
            }
        }
    }
}
