mod person;

fn main() {
    let person = person::Person::new(String::from("last"), String::from("first"), 5);
    person.display_person();
}
