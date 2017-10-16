extern crate rand;

use rand::Rng;

mod array_ins;

fn main() {
    let max_size = 100;
    let mut arr = array_ins::ArrayIns::new(max_size);
    let mut rng = rand::thread_rng();

    for i in 0..20 {
        arr.insert(rng.gen_range(0, 10));
    }

    arr.display();
    arr.no_dups();
    arr.display();
}
