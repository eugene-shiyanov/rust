extern crate rand;
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Угадайте число!");
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("пожалуйста, введите предположение.");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Не удалось прочитать строку");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        println!("Ваша попытка: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Слишком маленькое!"),
            Ordering::Greater => println!("Слишком большое!"),
            Ordering::Equal => {
                println!("Вы выиграли!");
                break;
            }
        }
    }
}
