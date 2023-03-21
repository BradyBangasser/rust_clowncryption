mod encryption;

use crate::encryption::encryption as Encryption;
use clap::{Command};

fn cli() -> Command {
    return Command::new("clown");
}

fn main() {
    let matches = cli().get_matches()

    println!("{:?}", args);
    println!("{}", Encryption::encrypt(&args.key, &args.message).unwrap());
    // println!("{}", Encryption::decrypt(&"hello world", &Encryption::encrypt(&"hello world", &"HELLO WORLD").unwrap()));
}
