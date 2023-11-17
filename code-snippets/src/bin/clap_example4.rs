/*
This is a simple application for Git operations that demonstrates usage of subcommands in clap.
It supports the following subcommands:

 - `clone`: Clone a repository.
    - Options:
        - `-n`, `--name`: Name of the repository to clone.

 - `push`: Push changes to a repository.
    - Options:
        - `-b`, `--branch`: Branch to push changes to.

 - `commit`: Commit changes to a repository.
    - Options:
        - `-m`, `--message`: Commit message.
*/
use clap::{Command, Arg};

fn main() {
    let app_m = Command::new("MyGitApp")
        .subcommand(Command::new("clone").about("Clone a repository").arg(
            Arg::new("name")
                .help("repo name")
                .short('n')
                .long("name")
                .num_args(1)
        ))
        .subcommand(Command::new("push").about("Push changes to a repository").arg(
            Arg::new("branch")
                .help("branch name")
                .short('b')
                .long("branch")
                .num_args(1)
        ))
        .subcommand(Command::new("commit").about("Commit changes to a repository").arg(
            Arg::new("message")
                .help("commit message")
                .short('m')
                .long("message")
                .num_args(1)
        ))
        .get_matches();


    match app_m.subcommand() {
        Some(("clone", sub_m)) => {
            // Handle clone subcommand
            if let Some(name) = sub_m.get_one::<String>("name") {
                println!("Cloning repository with name: {}", name);
            }
            else {
                println!("Please provide name of repositor to clone");
            }
        }
        Some(("push", sub_m)) => {
            // Handle push subcommand
            if let Some(branch) = sub_m.get_one::<String>("branch") {
                println!("Pushing changes to branch: {}", branch);
            }
            else {
                println!("Please provide the branch to push");
            }
        }
        Some(("commit", sub_m)) => {
            // Handle commit subcommand
            if let Some(message) = sub_m.get_one::<String>("message") {
                println!("Commiting changes with message: {}", message);
            }
            else {
                println!("Please provide a commit message");
            }
        }
        _ => {
            // Either no subcommand or invalid subcommand
            println!("Invalid subcommand or no subcommand provided");
        }
    }
}
