mod encryption;

use crate::encryption::encryption as Encryption;

fn main() {
    println!("{}", Encryption::encrypt(&"Hello World", &"Hello World").unwrap());
}
