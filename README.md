# 📇 Contact Manager CLI (Rust)

A simple command-line contact manager written in Rust.  
Supports adding, viewing, deleting contacts, saving to a JSON file, input validation, and timestamping.

---

## ✨ Features

- Add new contacts with name, phone, email, and created date.
- View all contacts sorted alphabetically.
- Delete contacts by name.
- Save and load contacts to/from `contacts.json`.
- Input validation:
  - Name: Alphabetic characters only.
  - Phone: Digits only, at least 10 digits.
  - Email: Valid email format using regex.
- Timestamp each contact with `created_at` using UTC time.

---

## 🛠 Installation

1. Make sure you have [Rust](https://www.rust-lang.org/tools/install) installed.

2. Clone this repository:

   ```bash
   git clone https://github.com/your-username/contact-manager-cli.git
   cd contact-manager-cli
   ```
3. Build and run:
    ```bash
    cargo run
    ```

## 📋Usage

When you run the program, you will see a menu:

1. Add a contact
2. View all contacts
3. Delete contact by name
4. Exit

- Add a contact: Enter name, phone number, and email. Data will be validated.

- View all contacts: Lists contacts alphabetically by name.

- Delete a contact: Enter the name of the contact to delete.

- Exit: Saves all changes to contacts.json.


## Data Persistance
- Contacts are automatically saved to a file called contacts.json in the project directory.

- On startup, if contacts.json exists, it loads all saved contacts.



## 🔧 Dependencies

This project uses the following crates:

- serde — serialization and deserialization.

- serde_json — JSON handling.

- regex — for phone and email validation.

- chrono — for timestamping contact creation.

Add them in Cargo.toml


## 👨‍💻 Author
- yaroCreates 


