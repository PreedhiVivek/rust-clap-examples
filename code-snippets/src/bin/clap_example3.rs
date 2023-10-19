/*
    A CLI application showing
        - grouping of command-line arguments
*/

use clap::{Command, Arg, ArgAction, ArgGroup};

fn main() {
    // Build the CLI by defining the configuration using builder pattern
    let cmd = Command::new("NumberCalculator")
        .version("1.0")
        .about("Calculates square or cube of a number based on choice of operation")
        .arg(
            Arg::new("number")
                .short('n')
                .long("number")
                .action(ArgAction::Set)
                .required(true)
        )
        .arg(
            Arg::new("square")
                .short('s')
                .long("square")
        )
        .arg(
            Arg::new("cube")
                .short('c')
                .long("cube")
        )
        .group(
            ArgGroup::new("operation")
                .args(&["square", "cube"])
                .required(true)
        );

    // Runtime argument parsing
    let matches = cmd.get_matches();
    let number = matches.get_one("number");

    let no = match number {
        Some(number) => number,
        None => &0,
    };

    if matches.contains_id("square") {
        println!("Square of {} is: {}", no, no * no);
    } else if matches.contains_id("cube") {
        println!("Cube of {} is: {}", no, no * no * no);
    }
}
