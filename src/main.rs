use std::error;

use clap::{App, Arg, SubCommand};

use etherise::eth1::*;
use rlp::decode;
use rustc_hex::FromHex;

fn main() -> Result<(), Box<dyn error::Error>> {
    let matches = App::new("etherise")
        .version("0.1.0")
        .author("Shoaib Ahmed <sufialhussaini@gmail.com>")
        .about("CLI toolkit for Ethereum")
        .arg(Arg::with_name("v")
            .short("v")
            .multiple(true)
            .help("Sets the level of verbosity"))
        .subcommand(SubCommand::with_name("rlp")
            .about("Recursive-length prefix (RLP)")
            .arg(Arg::with_name("decode")
                .short("d")
                .long("decode")
                .takes_value(true)
                .help("decode RLP data")))
        .get_matches();

    match matches.occurrences_of("v") {
        0 => println!("No verbose info"),
        1 => println!("Some verbose info"),
        2 => println!("Tons of verbose info"),
        3 | _ => println!("Don't be crazy"),
    }

    if let Some(matches) = matches.subcommand_matches("rlp") {
        if let Some(hex) = matches.value_of("decode") {
            let bytes = FromHex::from_hex(&hex[2..]).unwrap();
            let rlp = decode::<UnverifiedTransaction>(bytes.as_slice())?;
            println!("{:#?}", rlp);
        }
    }

    Ok(())
}