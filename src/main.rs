#[macro_use]
use clap::Clap;
use crate::days::day1;

#[derive(Clap)]
#[clap(version="1.0")]
struct Opts { 
    #[clap(short, long, parse(from_occurrences))]
    verbose: i32,

    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    #[clap(author="day 1 command")]
    Day1(true),
}

fn main() {
    let opts: Opts = Opts::parse();
    println!("Hello, world!");

    match opts.subcmd {
        SubCommand::Day1(d) => {
            println!("got day1 command")
        }
    }
}
