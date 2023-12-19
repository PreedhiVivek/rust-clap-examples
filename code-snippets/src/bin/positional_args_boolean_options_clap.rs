/*
    Program to show usage of positional arguments and boolean options
*/

use clap::{Command, Arg, ArgAction};

fn main(){
    // Build the CLI app using builder pattern
    let cmd = Command::new("NumberProgram")
    .version("1.0")
    .about("Program to show usage of positional argument and boolen option using clap")
    .arg(
        Arg::new("inputNumber")
        .help("a whole number")
        .index(1)
        .value_parser(clap::value_parser!(i32))
        .required(true)
    )
    .arg(
        Arg::new("square")
        .help("Boolean option to get the square of the input number")
        .short('s')
        .long("square")
        .required(false)
        .num_args(0)
        .action(ArgAction::Count)
    );

    // Runtime argument parsing
    let matches = cmd.get_matches();
    let number = matches.get_one("inputNumber");
    //Extract the value passed for the positional argument
    let no = match number {
        Some(number) => number,
        None => &0,
    };

    if matches.index_of("inputNumber") == Some(1){
        println!("Number passed via CLI : {} ", no);
    }

    let count = matches.get_count("square");
    if count>0 {
        println!("Square of {} : {} ", no, no*no);
    }
}
