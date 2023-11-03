/*
    A CLI application for temperature conversion
*/

use clap::{Command, Arg, ArgGroup};

fn main() {
    // Build the CLI by defining the configuration using builder pattern
    let cmd = Command::new("TemperatureConverter")
        .version("1.0")
        .about("Convert temperatures between Celsius and Fahrenheit")
        .arg(
            Arg::new("temperature")
                .short('t')
                .long("temperature")
                .num_args(1)
                .value_parser(clap::value_parser!(f64))
                .required(true)
        )
        .arg(
            Arg::new("celsius")
                .short('c')
                .long("celsius")
                .num_args(0)
        )
        .arg(
            Arg::new("fahrenheit")
                .short('f')
                .long("fahrenheit")
                .num_args(0)
        )
        .group(
            ArgGroup::new("conversion")
                .args(["celsius", "fahrenheit"])
                .required(true)
        );

    // Runtime argument parsing
    let matches = cmd.get_matches();
    let temperature = matches.get_one("temperature").unwrap();
    let to_celsius = matches.get_one("celsius").unwrap();

    if *to_celsius {
        let celsius = (temperature - 32.0) * 5.0 / 9.0;
        println!("{} Fahrenheit is {} Celsius", temperature, celsius);
    } else {
        let fahrenheit = (temperature * 9.0 / 5.0) + 32.0;
        println!("{} Celsius is {} Fahrenheit", temperature, fahrenheit);
    }
}
