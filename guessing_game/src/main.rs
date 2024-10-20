use std::io;

fn main(){
    println!("Enter the number");

    println!("Please input your number");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Fetal to read line");

    println!("you guessed: {}",guess);
}