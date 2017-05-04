pub struct Person {
    last_name: &str,
    first_name: &str,
    age: i32
}

impl Person {
    pub fn new(last: &str, first: &str , age: i32) -> Person {
        Person {
            last_name: last,
            first_name: first,
            age: age
        }
    }

    pub fn displayPerson(&self) {
        print!("Last name: {}", self.last_name);
        print!(", First name: {}", self.first_name);
        println!(", Age: {}", self.age);
    }
}
