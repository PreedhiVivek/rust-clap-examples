/*
    A CLI application showing
        - grouping of command-line arguments
*/
use clap::{Command, Arg, ArgGroup};

fn main() {
    // Build the CLI by defining the configuration using builder pattern
    let cmd = Command::new("NumberCalculator")
        .version("1.0")
        .about("Calculates square or cube of a number based on choice of operation")
        .arg(
            Arg::new("number")
                .short('n')
                .long("number")
                .num_args(1)
                .value_parser(clap::value_parser!(i32))
                .required(true)
        )
        .arg(
            Arg::new("square")
                .short('s')
                .long("square")
                .num_args(0)
        )
        .arg(
            Arg::new("cube")
                .short('c')
                .long("cube")
                .num_args(0)
        )
        .group(
            ArgGroup::new("operation")
                .args(["square", "cube"])
                .required(true)
        );

    // Runtime argument parsing
    let matches = cmd.get_matches();

    // Parse and store the number passed
    let number_passed = matches.get_one("number");
    let number = match number_passed {
        Some(number_passed) => number_passed,
        None => &0,
    };

    // Collect all the arguments passed into a Vector
    let arguments = matches.ids()
            .map(|id| id.as_str())
            .collect::<Vec<_>>();

    //Iterate over the vector to find the chosen operation
    let cube_index = arguments.iter().position(|&val| val == "cube");
    let square_index = arguments.iter().position(|&val| val == "square");

    if cube_index < square_index {
        println!("Cube of {} is: {}", number, number * number * number);
    }
    else{
        println!("Square of {} is: {}", number, number * number);
    }
}
