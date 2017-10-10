mod person;
mod array_in_ob;

fn main() {
    let mut arr = array_in_ob::ArrayInOb::new();

    arr.insert("Evans", "Patty", 24);
    arr.insert("Smith", "Lorraine", 37);
    arr.insert("Yee", "Tom", 43);
    arr.insert("Adams", "Henry", 63);
    arr.insert("Hashimoto", "Sato", 21);
    arr.insert("Stimson", "Henry", 29);
    arr.insert("Velasques", "Jose", 72);
    arr.insert("Lamarque", "Henry", 54);
    arr.insert("Vang", "Minh", 22);
    arr.insert("Creswell", "Lucinda", 18);

    println!("Before sorting:");
    arr.display();
    arr.insertion_sort();
    println!("After sorting:");
    arr.display();
}
