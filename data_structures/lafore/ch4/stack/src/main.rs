mod stack;

use stack::StackX;

fn main() {
    let mut the_stack = StackX::new(10);

    the_stack.push(20);
    the_stack.push(40);
    the_stack.push(60);
    the_stack.push(80);

    while !the_stack.is_empty() {
        print!("{} ", the_stack.pop());
    }

    println!();
}
