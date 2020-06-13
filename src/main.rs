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
        .subcommand(SubCommand::with_name("rlp")
            .about("Recursive-length prefix (RLP)")
            .arg(Arg::with_name("decode")
                .short("d")
                .long("decode")
                .takes_value(true)
                .help("decode RLP data")))
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("rlp") {
        if let Some(hex) = matches.value_of("decode") {
            let bytes = FromHex::from_hex(&hex[2..]).unwrap();
            let rlp = decode::<UnverifiedTransaction>(bytes.as_slice())?;
            println!("{:#?}", rlp);
        }
    }

    Ok(())
}