/*
use std::fs::{File, create_dir_all};
use std::io::{self, Write};
use std::path::Path;
use std::fs;

fn get_next_state_filename(directory: &str) -> io::Result<String> {
    // Make sure the directory exists
    create_dir_all(directory)?;

    // Read the directory's contents and find the highest file number
    let mut max_number = -1;
    for entry in fs::read_dir(directory)? {
        let entry = entry?;
        let filename = entry.file_name().to_string_lossy();
        
        // Check if the filename matches the pattern "stateX"
        if filename.starts_with("state") {
            if let Some(number_str) = filename.trim_start_matches("state").parse::<i32>().ok() {
                max_number = max_number.max(number_str);
            }
        }
    }

    // Increment to the next number
    let next_number = max_number + 1;
    Ok(format!("{}/state{}", directory, next_number))
}

fn write_to_state_file(directory: &str, data: &[u8]) -> io::Result<()> {
    let filename = get_next_state_filename(directory)?;
    let mut file = File::create(filename)?;

    // Write data to the file
    file.write_all(data)?;

    Ok(())
}

fn main() -> io::Result<()> {
    // Directory where the state files will be saved
    let directory = "states";

    // Example data to write to the state file
    let data = b"Hello, world! This is a state file.";

    // Write to the next state file
    write_to_state_file(directory, data)?;

    Ok(())
}

}
*/
pub fn save_state(){

}