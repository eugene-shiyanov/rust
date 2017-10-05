use person::Person;

pub struct ArrayInOb {
    a: Vec<Person>,
}

impl ArrayInOb {
    pub fn new() -> ArrayInOb {
        ArrayInOb {
            a: Vec::new()
        }
    }

    pub fn insert(&mut self, last: &str, first: &str, age: u32) {
        self.a.push(Person::new(last, first, age));
    }
}