mod reverser;
mod stack;

use reverser::Reverser;
use std::io;
use std::io::Write;

fn main() {
    loop {
        print!("Enter a string: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.eq("") {
            break;
        }

        let reverser = Reverser::new(&input);
        println!("Reversed: {}", reverser.do_rev());

    }
}
