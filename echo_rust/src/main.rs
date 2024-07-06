use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Do not print the trailing newline character
    #[arg(short, default_value_t = false)]
    n: bool,
}


fn main() {
    println!("Hello, world!");
    let args = Args::parse();
    println!("{:?}", args);

}
