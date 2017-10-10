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

    pub fn display(&self) {
        for person in &self.a {
            person.display_person();
        }

        println!();
    }

    pub fn insertion_sort(&mut self) {
        for out in 1..self.a.len() {
            let mut i = out;

            while i > 0 && self.a[i].get_last().lt(self.a[i - 1].get_last()) {
                self.a.swap(i - 1, i);
                i -= 1;
            }
        }
    }
}