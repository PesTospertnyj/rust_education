use std::io;

fn main() {
    println!("Угадайте число!");

    println!("Введите предположение.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("не удалось прочитать строку");

    println!("Ваша попытка: {}", guess);
}
