mod array_ins;

fn main() {
    let max_size = 100;
    let mut arr = array_ins::ArrayIns::new(max_size);

    for i in (0..10).rev() {
        arr.insert(i);
    }

    arr.display();
    arr.insertion_sort();
    arr.display();
    println!();

    let mut arr = array_ins::ArrayIns::new(max_size);

    for i in (0..100).rev() {
        arr.insert(i);
    }

    arr.display();
    arr.insertion_sort();
    arr.display();
    println!();

    let mut arr = array_ins::ArrayIns::new(max_size);

    for i in 0..10 {
        arr.insert(i);
    }

    arr.display();
    arr.insertion_sort();
    arr.display();
    println!();

    let mut arr = array_ins::ArrayIns::new(max_size);

    for i in 0..100 {
        arr.insert(i);
    }

    arr.display();
    arr.insertion_sort();
    arr.display();
    println!();
}
