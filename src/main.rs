mod encryption;

use crate::encryption::encryption as Encryption;
use clap::{Command};

fn main() {
    let cmd = clap::Command::new("clowncryption")
        .bin_name("clown")
        .subcommand_required(true)
        .subcommand(
            clap::command!("encrypt").arg(
                clap::arg!([MESSAGE])
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .help("The message to encrypt")
            ).about("Encrypt a message")
        )
        .subcommand(
            clap::command!("decrypt").arg(
                clap::arg!([KEY])
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .help("The key for the encrypted message")
                    .required(true)
            ).arg(
                clap::arg!([MESSAGE])
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .help("The encrypted message")
                    .required(true)
            ).about("PENIS")
        );
    let matches = cmd.get_matches();
    let matches = match matches.subcommand() {
        Some(("encrypt", matches)) => matches,
        _ => unreachable!("clap should ensure we don't get here"),
    };
    let manifest_path = matches.get_one::<std::path::PathBuf>("manifest-path");
    println!("{:?}", manifest_path);
}