use clap::{Arg, ArgAction, Command};

// #[derive(Parser, Debug)]
// #[command(version, about, long_about = None)]
// struct Cli {
//     /// Do not print the trailing newline character
//     #[arg(short, default_value_t = false)]
//     n: bool,
// }

fn main() {
    println!("Hello, world!");
    // let cli = Cli::parse();
    let matches = Command::new("echo_rust")
        .bin_name("echo_rust")
        .arg(
            Arg::new("text")
                .value_name("TEXT")
                .help("input text")
                .required(false)
                .action(ArgAction::Append)
                .num_args(0..),
        )
        .arg(
            Arg::new("omit_newline")
                .help("do not print the trailing newline character")
                .short('n')
                .action(ArgAction::SetTrue),
        )
        .get_matches();
    // println!("{:?}", cli);
    println!("{:?}", matches);
    let omit_newline = matches.get_flag("omit_newline");
    println!("omit_newline? {}", omit_newline);
    let arguments: Vec<&String> = match matches.get_occurrences("text") {
        Some(occ) => occ.flat_map(|hmm| hmm.into_iter()).collect(),
        None => vec![],
    };
    println!("text: {:#?}", arguments);
}
