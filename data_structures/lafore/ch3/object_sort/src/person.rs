pub struct Person {
    last_name: String,
    first_name: String,
    age: u32
}

impl Person {
    pub fn new(last: &str, first: &str , age: u32) -> Person {
        Person {
            last_name: String::from(last),
            first_name: String::from(first),
            age: age
        }
    }

    pub fn display_person(&self) {
        print!("Last name: {}", self.last_name);
        print!(", First name: {}", self.first_name);
        println!(", Age: {}", self.age);
    }

    pub fn get_last(&self) -> &str {
        &self.last_name
    }
}
