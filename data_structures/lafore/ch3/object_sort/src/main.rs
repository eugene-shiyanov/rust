mod person;

fn main() {
    let person = person::Person::new("last", "first", 5);
    person.display_person();
}
