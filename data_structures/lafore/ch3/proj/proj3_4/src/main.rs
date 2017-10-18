extern crate rand;

use rand::Rng;

mod array_bub;

fn main() {
    let max_size = 100;
    let mut arr = array_bub::ArrayBub::new(max_size);

    let mut rng = rand::thread_rng();

    for _ in 0..50 {
        arr.insert(rng.gen_range(0, 50));
    }

    arr.display();
    arr.odd_even_sort();
    arr.display();

}
