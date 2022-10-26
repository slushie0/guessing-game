use std::io::*;
use rand::Rng;

fn main() {
    let ans = rand::thread_rng().gen_range(0..10);
    let mut guess: i32 = -1;
    let mut input;

    println!("Guess a number between 1 and 10");

    while guess != ans {
        input = String::new();
        stdin().read_line(&mut input).expect("error: unable to read user input");
        
        let guess_str = input.trim();
        match guess_str.parse::<u32>() {
            Ok(i) => guess = i as i32,
            Err(..) => guess = -1,
        };
        println!("Guess again");
    }
    println!("You win! The number was {}.", ans);
}
