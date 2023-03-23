mod encryption;

use crate::encryption::encryption as Encryption;
use clap;

fn main() {
    let cmd = clap::Command::new("clowncryption")
        .bin_name("clown")
        .subcommand_required(true)
        .subcommand(
            clap::command!("encrypt")
            .arg(
                clap::arg!(key: [KEY])
                    .value_parser(clap::value_parser!(String))
                    .help("The key for the encrypted message")
                    .required(true)
            )
            .arg_required_else_help(true).arg(
                clap::arg!(message: [MESSAGE])
                    .value_parser(clap::value_parser!(String))
                    .help("The message to encrypt")
                    .required(true)
            )
            .about("Encrypt a message")
            .arg_required_else_help(true)
            .arg(clap::arg!(binary: -b --binary)
            .action(clap::ArgAction::SetTrue)
            .help("Encode message in binary"))
            .arg(
                clap::arg!(nonce: --nonce <NONCE>)
                .help("Sets the Nonce")
                .default_value("nonce")
            )
            .arg(clap::arg!(encode: -d --dontencode)
            .action(clap::ArgAction::SetFalse)
            .help("Don't encode the message in clowns"))

        )
        .subcommand(
            clap::command!("decrypt")
            .arg(
                clap::arg!(key: [KEY])
                    .value_parser(clap::value_parser!(String))
                    .help("The key for the encrypted message")
                    .required(true)
            )
            .arg_required_else_help(true)
            .arg(
                clap::arg!(message: [MESSAGE])
                    .value_parser(clap::value_parser!(String))
                    .help("The encrypted message")
                    .required(true)
            )
            .arg_required_else_help(true)
            .about("Decrypt a message")
            .arg(clap::arg!(binary: -b --binary)
            .action(clap::ArgAction::SetTrue)
            .help("Decodes message from binary"))
            .arg(
                clap::arg!(nonce: --nonce <NONCE>)
                .help("Sets the Nonce")
                .default_value("nonce")
            )
            .arg(clap::arg!(encode: -d --dontencode)
            .action(clap::ArgAction::SetFalse)
            .help("Don't encode the message in clowns"))
        );


    let matches = cmd.get_matches();
    let encrypted_string: String = match matches.subcommand() {
        Some(("encrypt", matches)) => {
            let key = matches.get_one::<std::string::String>("key").unwrap();
            let message = matches.get_one::<std::string::String>("message").unwrap();
            let binary = matches.get_one::<bool>("binary").unwrap();
            let encode = matches.get_one::<bool>("encode").unwrap();
            let nonce = matches.get_one::<std::string::String>("nonce").unwrap();
            Encryption::encrypt(key, message, &mut nonce.as_bytes().to_vec(), *encode, *binary).unwrap()
        },
        Some(("decrypt", matches)) => {
            let key = matches.get_one::<std::string::String>("key").unwrap();
            let message = matches.get_one::<std::string::String>("message").unwrap();
            let binary = matches.get_one::<bool>("binary").unwrap();
            let nonce = matches.get_one::<std::string::String>("nonce").unwrap();
            let encode = matches.get_one::<bool>("encode").unwrap();
            Encryption::decrypt(key, message, &mut nonce.as_bytes().to_vec(), *encode, *binary)
        },
        _ => unreachable!("clap should ensure we don't get here"),
    };
    println!("{}", encrypted_string);
}