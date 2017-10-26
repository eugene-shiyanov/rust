mod reverser;
mod stack;

use reverser::Reverser;

fn main() {
    let reverser = Reverser::new("hello");
    println!("{}", reverser.do_rev());
}
