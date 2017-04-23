fn main() {
    println!("fahr\tcel");
    print_degree_table(fahr_to_cel);
    println!("\ncel\tfahr");
    print_degree_table(cel_to_fahr);
}

fn print_degree_table(converter : fn(f32) -> f32) {
    let mut i = 0;

    while i <= 100 {
        println!("{}\t{:.1}", i, converter(i as f32));
        i += 10;
    }
}

fn fahr_to_cel(fahr : f32) -> f32 {
    (fahr - 32f32) * 5f32 / 9f32 
}

fn cel_to_fahr(cel : f32) -> f32 {
    cel * 9f32 / 5f32 + 32f32
}
