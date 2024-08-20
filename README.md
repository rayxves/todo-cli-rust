# Todo CLI

A simple command-line todo list application built with Rust. 

## Project Description

This project was created to learn about file systems and the basics of serialization and deserialization in Rust. By building this command-line todo list application, I aimed to gain practical experience with Rust's capabilities in handling file operations and data management.

## Features

- **Add a new task** with a specified name and completion date.
- **View all tasks** that are yet to be completed.
- **Remove tasks** by name.
- **Update task details** including name and completion time.
- **Mark tasks as completed** and move them to a separate list.
- **View completed tasks** at any time.

## Prerequisites

- Rust (1.64 or higher)
- Cargo (comes with Rust)

## Installation

1. **Clone the repository:**

    ```bash
    git clone https://github.com/rayxves/todo-cli-rust.git 
    cd todo-cli-rust
    ```

2. **Build the project:**

    ```bash
    cargo build
    ```

## Usage

### Adding a Task

```cargo run -- --add-task "Task Name" "Completion Date" ```

### Viewing All Tasks

```cargo run -- --view-tasks="true" ```

### Removing a Task

```cargo run -- --remove-task "Task Name"```

### Updating a Task's Name

```cargo run -- --update-name "Old Task Name" "New Task Name" ```

### Updating a Task's Completion Time

```cargo run -- --update-concluded-time "Task Name" "New Completion Date" ```

### Marking a Task as Completed

```cargo run -- --concluded-task "Task Name" ```

### Viewing Completed Tasks

 ```cargo run -- --view-concluded-tasks="true"```