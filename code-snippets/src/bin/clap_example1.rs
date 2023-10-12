use clap::Parser;
/// Calculate the square and cube of a number
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// The number for which to calculate square and cube
    #[arg(short, long)]
    number: f64,
}

fn main() {
    let args = Args::parse();

    let square = args.number * args.number;
    let cube = args.number * args.number * args.number;

    println!("Number: {}", args.number);
    println!("Square of number: {}", square);
    println!("Cube of number: {}", cube);
}