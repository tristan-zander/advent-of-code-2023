use clap::Parser;
use macros::solutions;

#[macro_use]
extern crate anyhow;

mod day_four;
mod day_one;
mod day_three;
mod day_two;
mod day_five;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(long, short, value_parser = clap::value_parser!(u8).range(1..=25))]
    day: u8,
    #[arg(long, short, value_parser = clap::value_parser!(u8).range(1..=2))]
    part: u8,
}

pub type Solution = fn(Args) -> ();

const SOLUTIONS: phf::Map<&'static str, Solution> = solutions![
    mod day_one;
    mod day_two;
    mod day_three;
    mod day_four;
    mod day_five;
];

fn main() {
    let args = Args::parse();
    SOLUTIONS
        .get(format!("{}.{}", args.day, args.part).as_str())
        .unwrap()(args);
}
