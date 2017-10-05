mod person;
mod array_in_ob;

fn main() {
    let person = person::Person::new("last", "first", 5);
    person.display_person();
}
