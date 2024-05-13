mod list;

use list::{print_list, append_item_to_file, remove_item_from_list, clear_list};
use std::env;

fn main() {
    let list: Vec<String> = env::args().collect();
    let file_path: &str = "list_to_do.txt";
    
    match list[1].as_str() {
        "print" => {
            match print_list(&file_path) {
                Ok(contents) => println!("{}", contents),
                Err(err) => println!("Error reading file {}", err),
            }
        },
        "add" => {
            match append_item_to_file(&file_path, &list[2]) {
                Ok(()) => println!("Successfully added value {} to file.", list[2]),
                Err(e) => eprintln!("Error appending value to file: {}", e),
            }
        },
        "remove" => {
            match remove_item_from_list(file_path, &list[2]) {
                Ok(()) => println!("Successfully remove value {} from file.", list[2]),
                Err(e) => eprintln!("Error removing value from list{}", e),
            }
        },
        "clear" => {
            match clear_list(file_path) {
                Ok(()) => println!("Successfully, list clear"),
                Err(e) => eprintln!("Error clearin values from list{}", e),
            }
        }
        _ => (),
    }
}
