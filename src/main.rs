#[macro_use]
extern crate clap;
extern crate nom;

use clap::{App, Arg};

mod day1;
mod helper;

fn main() {
    // create cli app
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .long_about(crate_description!())
        .arg(Arg::with_name("day").index(1).help("Day of the challenge"))
        .arg(Arg::with_name("filename").index(2).help("Input file"))
        .get_matches();

    let input = matches.value_of("filename").expect("Insert a filename");

    match matches.value_of("day") {
        Some("1") => day1::handle_day(input),
        _ => panic!("Insert a valid day"),
    }
}
