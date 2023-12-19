/*
    A simple program showing
        - usage of mandatory and optional CLI arguments
        - defining short and long argument flags
*/
use clap::Parser;
use std::env;

///Login validation

#[derive(Parser, Debug)]
struct Args {
    /// User Name - optional argument
    #[arg(short = 'n', long ="UserName", default_value = "Qxf2")]
    user_name: String,

    /// Password - mandatory argument
    #[arg(short = 'p', long = "Password", required = true)]
    password: String,

}

fn main() {
    let args = Args::parse();
    let expected_password = match env::var("APP_PASSWORD") {
        Ok(val) => val,
        Err(_) => {
            println!("Password not set as an environment variable!");
            return;
        }
    };

    if args.password == expected_password{
        println!("Welcome to {}!", args.user_name);
    }
    else {
        println!("Password mismatch for user: {}! \nLogin failed!", args.user_name);
    }
}