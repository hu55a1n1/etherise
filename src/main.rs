use std::error;

use clap::{App, Arg};

fn main() -> Result<(), Box<dyn error::Error>> {
    let matches = App::new("etherise")
        .version("0.1.0")
        .author("Shoaib Ahmed <sufialhussaini@gmail.com>")
        .about("CLI toolkit for Ethereum")
        .arg(Arg::with_name("v")
            .short("v")
            .multiple(true)
            .help("Sets the level of verbosity"))
        .get_matches();

    match matches.occurrences_of("v") {
        0 => println!("No verbose info"),
        1 => println!("Some verbose info"),
        2 => println!("Tons of verbose info"),
        3 | _ => println!("Don't be crazy"),
    }

    Ok(())
}