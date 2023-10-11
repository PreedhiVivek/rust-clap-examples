/*
    A simple program prompting for login credentials, showing
        - mandatory and optional CLI arguments
        - defining short and long argument flags
*/
use clap::Parser;

///Enter Login Credentials
#[derive(Parser, Debug)]
struct Args {
    /// User Name
    #[arg(short = 'n', long ="UserName", default_value = "Qxf2")]
    user_name: String,

    /// Password - mandatory argument
    #[arg(short = 'p', long = "Password", required = true)]
    password: String,

}

fn main() {
    let args = Args::parse();
    println!("Welcome to {}!", args.user_name)
}