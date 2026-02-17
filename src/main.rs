use std::io::{self, Write};
use chrono::prelude::*; // for date handling

// A habit entry with a date and text
struct Entry {
    date: String,
    text: String,
}

// A habit struct with a name and a vector of entries
struct Habit {
    name: String,
    entries: Vec<Entry>,
}

impl Habit {
    // Create a new habit
    fn new(name: String) -> Habit {
        Habit {
            name,
            entries: Vec::new(),
        }
    }

    // Add an entry to the habit
    fn add_entry(&mut self, text: String) {
        let date = Local::now().format("%Y-%m-%d").to_string();
        self.entries.push(Entry { date, text });
    }

    // List all entries of this habit
    fn list_entries(&self) {
        if self.entries.is_empty() {
            println!("No entries yet for '{}'.", self.name);
        } else {
            for entry in &self.entries {
                println!("{}: {}", entry.date, entry.text);
            }
        }
    }
}

// Helper function to read input from the user
fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

// Main menu function
fn main() {
    let mut habits: Vec<Habit> = Vec::new();

    loop {
        println!("\n--- Habit Tracker ---");
        println!("1. Add a habit");
        println!("2. Add an entry");
        println!("3. List all habits");
        println!("4. List all entries of a habit");
        println!("5. Exit");

        let choice = read_input("Choose an option: ");

        match choice.as_str() {
            "1" => {
                let name = read_input("Enter habit name: ");
                habits.push(Habit::new(name));
                println!("Habit added!");
            }
            "2" => {
                if habits.is_empty() {
                    println!("No habits found. Add one first.");
                    continue;
                }
                println!("Select a habit:");
                for (i, habit) in habits.iter().enumerate() {
                    println!("{}: {}", i + 1, habit.name);
                }
                let index: usize = read_input("Enter number: ")
                    .parse()
                    .unwrap_or(0);
                if index == 0 || index > habits.len() {
                    println!("Invalid selection.");
                    continue;
                }
                let entry_text = read_input("Enter your entry: ");
                habits[index - 1].add_entry(entry_text);
                println!("Entry added!");
            }
            "3" => {
                if habits.is_empty() {
                    println!("No habits yet.");
                } else {
                    println!("Your habits:");
                    for habit in &habits {
                        println!("- {}", habit.name);
                    }
                }
            }
            "4" => {
                if habits.is_empty() {
                    println!("No habits found.");
                    continue;
                }
                println!("Select a habit to view entries:");
                for (i, habit) in habits.iter().enumerate() {
                    println!("{}: {}", i + 1, habit.name);
                }
                let index: usize = read_input("Enter number: ")
                    .parse()
                    .unwrap_or(0);
                if index == 0 || index > habits.len() {
                    println!("Invalid selection.");
                    continue;
                }
                habits[index - 1].list_entries();
            }
            "5" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid option, try again."),
        }
    }
}