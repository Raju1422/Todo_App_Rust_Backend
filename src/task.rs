// Importing the necessary crates and traits
use csv::{Reader, Writer}; // CSV reader and writer for handling CSV I/O
use serde::{Deserialize, Serialize}; // For serializing/deserializing structs to/from CSV
use std::fs::File; // For handling file operations
use std::io::Read; // To read file contents into a string

// Define the Task struct with Serde traits to allow serialization/deserialization
#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub name: String,
    pub description: String,
    pub complete: bool,
}

// Function to load tasks from a CSV file and return a vector of Task structs
pub fn load_tasks() -> Vec<Task> {
    // Create a mutable vector to store the loaded tasks
    let mut tasks = vec![];

    // Try to open the "tasks.csv" file; if it doesn't exist, create an empty one
    let mut file = File::open("tasks.csv").unwrap_or_else(|_| File::create("tasks.csv").unwrap());

    // String buffer to store the contents of the file
    let mut contents = String::new();

    // Try reading the file contents into the string buffer
    match file.read_to_string(&mut contents) {
        Ok(_) => {
            // Create a CSV reader from the string buffer's bytes
            let mut reader = Reader::from_reader(contents.as_bytes());

            // Iterate over each deserialized row in the CSV
            for result in reader.deserialize() {
                // Unwrap the result into a Task struct and push it into the tasks vector
                let task: Task = result.unwrap();
                tasks.push(task);
            }
        }
        // If reading the file fails, do nothing (e.g., file was empty)
        Err(_) => (),
    }

    // Return the loaded tasks
    tasks
}

// Function to save a list of Task structs into the CSV file
pub fn save_tasks(tasks: &Vec<Task>) {
    // Create (or overwrite) the "tasks.csv" file
    let file = File::create("tasks.csv").unwrap();

    // Create a CSV writer from the file
    let mut writer = Writer::from_writer(file);

    // Iterate through the list of tasks and serialize each one into the CSV file
    for task in tasks {
        writer.serialize(task).unwrap();
    }

    // Writer is automatically flushed when it goes out of scope
}
