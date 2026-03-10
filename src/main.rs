use std::io;
use std::process::Command;

enum FileOperation {
    List(String),
    Display(String),
    Create(String, String),
    Remove(String),
    Pwd,
}

fn perform_operation(operation: FileOperation) {
    match operation {
        FileOperation::List(dir) => {
            Command::new("ls")
                .arg(dir)
                .status()
                .expect("Failed to run ls");
        }

        FileOperation::Display(file) => {
            Command::new("cat")
                .arg(file)
                .status()
                .expect("Failed to run cat");
        }

        FileOperation::Create(file, content) => {
            let command = format!("echo '{}' > {}", content, file);

            Command::new("sh")
                .arg("-c")
                .arg(command)
                .status()
                .expect("Failed to create file");
        }

        FileOperation::Remove(file) => {
            Command::new("rm")
                .arg(file)
                .status()
                .expect("Failed to remove file");
        }

        FileOperation::Pwd => {
            Command::new("pwd")
                .status()
                .expect("Failed to run pwd");
        }
    }
}

fn main() {
    loop {
        println!("\nFile Operations Menu:");
        println!("1. List files in a directory");
        println!("2. Display file contents");
        println!("3. Create a new file");
        println!("4. Remove a file");
        println!("5. Print working directory");
        println!("0. Exit");

        let mut choice = String::new();
        println!("Enter your choice (0-5):");
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                let mut dir = String::new();
                println!("Enter directory path:");
                io::stdin().read_line(&mut dir).unwrap();
                perform_operation(FileOperation::List(dir.trim().to_string()));
            }

            "2" => {
                let mut file = String::new();
                println!("Enter file path:");
                io::stdin().read_line(&mut file).unwrap();
                perform_operation(FileOperation::Display(file.trim().to_string()));
            }

            "3" => {
                let mut file = String::new();
                let mut content = String::new();

                println!("Enter file path:");
                io::stdin().read_line(&mut file).unwrap();

                println!("Enter content:");
                io::stdin().read_line(&mut content).unwrap();

                perform_operation(FileOperation::Create(
                    file.trim().to_string(),
                    content.trim().to_string(),
                ));
            }

            "4" => {
                let mut file = String::new();
                println!("Enter file path:");
                io::stdin().read_line(&mut file).unwrap();
                perform_operation(FileOperation::Remove(file.trim().to_string()));
            }

            "5" => {
                perform_operation(FileOperation::Pwd);
            }

            "0" => {
                println!("Goodbye!");
                break;
            }

            _ => println!("Invalid option"),
        }
    }
}