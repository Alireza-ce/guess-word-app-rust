use std::io;
use rand::Rng;


fn main() {

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Guess the number!{}", secret_number);

    println!("Please input your guess.");

    loop {
    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read line");

    let guess: u32 =match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("wrong format: please enter a number");
            continue
        },
    };

    println!("you passed {}", guess);

    if guess == secret_number {
        println!("you win dude");
        break;
    }else if guess > secret_number {
        println!("it's smaller")
    }else{
        println!("it's bigger")
    }

}
}
