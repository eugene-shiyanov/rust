mod array_ins;

fn main() {
    let max_size = 100;
    let mut arr = array_ins::ArrayIns::new(max_size);

    arr.insert(77);
    arr.insert(99);
    arr.insert(44);
    arr.insert(55);
    arr.insert(22);
    arr.insert(88);
    arr.insert(11);
    arr.insert(00);
    arr.insert(66);
    arr.insert(33);

    arr.display();
//    arr.insertion_sort();
//    arr.display();

    println!("median is {}", arr.median());
}
