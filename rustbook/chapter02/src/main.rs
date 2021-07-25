use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Guess a num");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        // let guessed_number: i32 = guess.parse().unwrap();

        // println!("You guessed: {}", guess);
        // if guessed_number != secret_number {
        //     println!("The number was actually {}", secret_number);
        // } else {
        //     println!("Correct!");
        // }

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You got it right");
                break;
            }
        }
    }
}
