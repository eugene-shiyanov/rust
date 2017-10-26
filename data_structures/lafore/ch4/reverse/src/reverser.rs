use stack::StackX;

pub struct Reverser {
    input: String
}

impl Reverser {
    pub fn new(input: &str) -> Reverser {
        Reverser {
            input: String::from(input)
        }
    }

    pub fn do_rev(&self) -> String {
        let chars: Vec<char> = self.input.chars().collect();
        let mut stack = StackX::new(chars.len());

        for ch in chars {
            stack.push(ch);
        }

        let mut output = String::new();

        while !stack.is_empty() {
            output.push(stack.pop());
        }

        output
    }
}