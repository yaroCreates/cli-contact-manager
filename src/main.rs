use chrono::{DateTime, Utc};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::{self, Write},
    path::Path,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Contact {
    name: String,
    phone: String,
    email: String,
    created_at: DateTime<Utc>,
}

const FILE_PATH: &str = "contacts.json";

fn main() {
    let mut contacts = load_contacts();

    loop {

        println!("\n");
        println!("Welcome to my Contact Manager");
        println!("\n");
        println!("1. Add a contact");
        println!("2. View all contacts");
        println!("3. Delete contact by name");
        println!("4. Exit");

        print!("Enter choice: ");
        io::stdout().flush().unwrap();
        let choice = read_input();

        match choice.as_str() {
            "1" => {
                add_contact(&mut contacts);
                save_contacts(&contacts);
            }
            "2" => view_contacts(&mut contacts),
            "3" => {
                delete_contact(&mut contacts);
                save_contacts(&contacts);
            }
            "4" => {
                println!("Hope to see you back soon!");
                break;
            }
            _ => println!("Invalid selection, please choose from 1-4."),
        }
    }
}

fn add_contact(contacts: &mut Vec<Contact>) {
    println!("\n--- Add Contact ---");

    let name = loop {
        print!("Enter name: ");
        io::stdout().flush().unwrap();
        let input = read_input();
        if validate_name(&input) {
            break input;
        } else {
            println!("Name must be alphabetic and non-empty.");
        }
    };

    let phone = loop {
        print!("Enter phone: ");
        io::stdout().flush().unwrap();
        let input = read_input();
        if validate_phone(&input) {
            break input;
        } else {
            println!("Phone must be digits only and at least 10 digits.");
        }
    };

    let email = loop {
        print!("Enter email: ");
        io::stdout().flush().unwrap();
        let input = read_input();
        if validate_email(&input) {
            break input;
        } else {
            println!("Invalid email format.");
        }
    };

    contacts.push(Contact {
        name,
        phone,
        email,
        created_at: Utc::now(),
    });

    println!("Contact added.");
}

fn view_contacts(contacts: &mut Vec<Contact>) {
    println!("\n--- All Contacts ---");

    if contacts.is_empty() {
        println!("No contacts found.");
    } else {
        contacts.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
        for (i, c) in contacts.iter().enumerate() {
            println!(
                "{}. Name: {}, Phone: {}, Email: {}, Created At: {}",
                i + 1,
                c.name,
                c.phone,
                c.email,
                c.created_at
            );
        }
    }
}

fn delete_contact(contacts: &mut Vec<Contact>) {
    println!("\n--- Delete Contact ---");
    print!("Enter name to delete: ");
    io::stdout().flush().unwrap();
    let name = read_input();

    let initial_len = contacts.len();
    contacts.retain(|c| c.name.to_lowercase() != name.to_lowercase());

    if contacts.len() < initial_len {
        println!("Contact deleted.");
    } else {
        println!("Contact not found.");
    }
}

fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

// Validation

fn validate_name(name: &str) -> bool {
    !name.trim().is_empty() && name.chars().all(|c| c.is_alphabetic() || c.is_whitespace())
}

fn validate_phone(phone: &str) -> bool {
    let re = Regex::new(r"^\d{10,}$").unwrap();
    re.is_match(phone)
}

fn validate_email(email: &str) -> bool {
    let re = Regex::new(r"^[\w\.-]+@[\w\.-]+\.\w+$").unwrap();
    re.is_match(email)
}

// JSON File Handling

fn load_contacts() -> Vec<Contact> {
    if Path::new(FILE_PATH).exists() {
        let data = fs::read_to_string(FILE_PATH).unwrap_or_default();
        serde_json::from_str(&data).unwrap_or_else(|_| Vec::new())
    } else {
        Vec::new()
    }
}

fn save_contacts(contacts: &Vec<Contact>) {
    let json = serde_json::to_string_pretty(contacts).unwrap();
    let mut file = File::create(FILE_PATH).unwrap();
    file.write_all(json.as_bytes()).unwrap();
}
