extern crate rand;

use rand::Rng;

mod array_ins;

fn main() {
    let mut rng = rand::thread_rng();

    let max_size = 100;
    let mut arr = array_ins::ArrayIns::new(max_size);

    for _ in 0..20 {
        arr.insert(rng.gen_range(0, 10));
    }

    arr.display();
    arr.insertion_sort();
    arr.display();
}
