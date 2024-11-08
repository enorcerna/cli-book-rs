use clap::Parser;
#[derive(Parser, Debug)]
struct Cli {
    #[arg(short, long)]
    input: String,
}

fn main() {
    let args = Cli::parse();
    println!("text: {}", args.input);
}
