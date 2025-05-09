# 📝 Todo App (Rust + Rocket)

This is a simple **Todo List REST API** built using [Rust](https://www.rust-lang.org/) and the [Rocket](https://rocket.rs/) web framework. It allows you to create, read, update, and delete tasks, which are persisted in a local CSV file (`tasks.csv`).

---

## 🚀 Features

- Create a new task
- Fetch all tasks
- Update an existing task
- Delete a task
- Persist tasks using CSV

---

## 📦 Dependencies

- [Rocket](https://rocket.rs/) - Web framework for Rust
- [Serde](https://serde.rs/) - Serialization framework
- [CSV](https://docs.rs/csv/) - For reading/writing CSV files

---

## 📁 Project Structure

Todo_App/  
├── Cargo.toml # Project manifest file with dependencies  
├── main.rs # Main application file with Rocket routes  
├── task.rs # Task struct definition and CSV utilities  
└── tasks.csv # Auto-created on first run to persist tasks

---

## ⚙️ How It Works

### Endpoints

| Method | Endpoint       | Description             |
| ------ | -------------- | ----------------------- |
| GET    | `/tasks`       | Get all tasks           |
| POST   | `/create-task` | Create a new task       |
| PUT    | `/update-task` | Update an existing task |
| DELETE | `/delete-task` | Delete an existing task |

> All non-GET endpoints accept JSON data.

### Sample Task JSON

```json
{
  "name": "Learn Rust",
  "description": "Finish Rocket tutorial",
  "complete": false
}
```

## ▶️ Running the App

### 1. Clone the Repository

```bash
git clone https://github.com/Raju1422/Todo_App_Rust_Backend.git
cd Todo_App
```

### 2. Run the App

```bash
cargo run
```

The server will launch (by default) at:  
📍 http://localhost:8000

## 📌 Notes

- The unique identifier for a task is its name. No two tasks can have the same name.
- The CSV file is auto-created as tasks.csv in the project root.

## 🛠 Built With

- 🦀 Rust

- 🚀 Rocket

- 🧬 Serde

- 📄 CSV crate
