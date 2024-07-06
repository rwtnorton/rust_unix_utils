use clap::{Arg, ArgAction, ArgMatches, Command};

fn main() {
    let matches = cli().get_matches();
    let args = Args::from_matches(&matches);
    println!("args: {:#?}", args);
    println!("omit_newline: {}", args.omit_newline);
    println!("arguments: {:?}", args.arguments);
}

fn cli() -> Command {
    Command::new("echo_rust")
        .bin_name("echo_rust")
        .about("write arguments to the standard output")
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
}

#[derive(Debug)]
struct Args<'a> {
    omit_newline: bool,
    arguments: Vec<&'a String>,
}

impl<'a> Args<'a> {
    pub fn from_matches(matches: &'a ArgMatches) -> Self {
        let omit_newline = matches.get_flag("omit_newline");
        let arguments: Vec<&String> = match matches.get_occurrences("text") {
            Some(occ) => occ.flat_map(|hmm| hmm.into_iter()).collect(),
            None => vec![],
        };
        Args {
            omit_newline,
            arguments,
        }
    }
}
