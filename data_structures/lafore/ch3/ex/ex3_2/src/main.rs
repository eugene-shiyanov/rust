use std::time;

mod array_bub;
mod array_sel;
mod array_ins;

fn main() {
    let max_size = 100_000;
    let mut arr_bub = array_bub::ArrayBub::new(max_size);
    let mut arr_sel = array_sel::ArraySel::new(max_size);
    let mut arr_ins = array_ins::ArrayIns::new(max_size);

    for i in (0..max_size).rev() {
        arr_bub.insert(i as i64);
        arr_sel.insert(i as i64);
        arr_ins.insert(i as i64);
    }

    println!("Bubble sorting...");
    let time = time::SystemTime::now();
    arr_bub.bubble_sort();
    println!("Time elapsed {}", time.elapsed().unwrap().as_secs());

    println!("Selection sorting...");
    let time = time::SystemTime::now();
    arr_sel.selection_sort();
    println!("Time elapsed {}", time.elapsed().unwrap().as_secs());

    println!("Insertion sorting...");
    let time = time::SystemTime::now();
    arr_ins.insertion_sort();
    println!("Time elapsed {}", time.elapsed().unwrap().as_secs());
}
