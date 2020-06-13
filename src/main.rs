use std::error;

use clap::{App, Arg, SubCommand};
use rustc_hex::FromHex;

use etherise::eth1::*;

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
            let bytes = &hex[2..].from_hex().unwrap();
            let rlp = EthRlp::new(bytes.as_slice());
            let err = rlp.to_json().unwrap();
            println!("{}", err);
        }
    }

    Ok(())
}