use std::io; // Import needed to receive input from the command line

// Struct to represent a todo item (feel free to edit as needed)
struct Todo {
    id: i16,
    title: String,
    completed: bool,
    deleted: bool,
}

// Helper function to print the todo list
fn print_todos(todos: &Vec<Todo>) {
    println!("\n\nTodo List:\n-------------------");
    for todo in todos {
        if !todo.deleted {
            let done = if todo.completed { "✔" } else { " " };
            println!("[{}] {} {}", done, todo.id, todo.title);
        }
    }
}

fn main() {
    // Empty vector/List to hold your todos
    let mut todos: Vec<Todo> = Vec::new();
    print_todos(&todos);

    loop {
        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("failed to read line");

        // text input split by spaces into a vector, so if command is:
        // `add buy milk`
        // command_parts[0] will be "add", command_parts[1] will be "buy", and so on.
        // Feel free to parse input in any way you want, this is to help you get started
        let command_parts: Vec<&str> = command.split_whitespace().collect();

        // Recognize commands and do something with the `todos` vector here

        // At the end of each loop print the list
        print_todos(&todos);
    }
}
