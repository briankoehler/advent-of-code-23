use std::path::Path;
use clap::Parser;
use macros::day_handler;
use day::day_1::Day1;
use day::day_2::Day2;

mod day;
mod utils;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    /// Day to execute
    #[arg(short, long)]
    day: u8,

    /// Part to execute
    #[arg(short, long)]
    part: u8,
}

pub trait Solution {
    fn new(filename: impl AsRef<Path>) -> Self;
    fn part_1(self) -> String;
    fn part_2(self) -> String;
}


fn main() {
    let args = Args::parse();

    let day = args.day;
    let part = args.part;
    day_handler!([
        (1, Day1),
        (2, Day2)
    ]);
}

