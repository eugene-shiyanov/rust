pub struct Person {
    last_name: String,
    first_name: String,
    age: i32
}

impl Person {
    pub fn new(last: String, first: String , age: i32) -> Person {
        Person {
            last_name: last,
            first_name: first,
            age: age
        }
    }

    pub fn display_person(&self) {
        print!("Last name: {}", self.last_name);
        print!(", First name: {}", self.first_name);
        println!(", Age: {}", self.age);
    }
}
