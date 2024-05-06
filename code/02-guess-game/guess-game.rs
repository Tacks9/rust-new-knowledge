use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("===Guessing-Number-Game===");

    let secret_number = rand::thread_rng().gen_range(1..101);
    let mut count:u32=0;
    loop {
        println!(">[1,100] Please Guess a number !");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Error Read number");
        let guess = guess.trim();
        if guess == "quit" {
            println!("GoodBye!");
            break;
        }
        let guess:u32 = match guess.parse() {
            Ok(num) => {
                if num < 1 || num > 100 {
                    println!(">? The number must be between 1 and 100.");
                    continue;
                } else {
                    num
                }
            },
            Err(_) => {
                println!(">? Invalid input. Please enter a number between 1 and 100, or 'quit' to exit.");
                continue;
            },
        };
        println!(">You Type a Number: {}" , guess);
        count = count + 1;
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!(">You Win! You guessed a total of {} times!", count);
                break;
            },
        }
    }
    

}