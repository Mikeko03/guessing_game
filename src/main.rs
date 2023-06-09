use std::io;
use std::cmp::Ordering;
use rand::Rng;


fn main(){
    println!("Welcome to guessing game.");
    
    
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Secret number is {secret_number}");
    
    loop{
        println!("Please input your guess");

        let mut guess = String::new();
   
        io::stdin()
            .read_line(&mut guess)
            .expect("Enter correct guess");

        let guess:u32 = match guess.trim().parse() {
            Ok(num)=>num,
            Err(_)=>continue,
        };

        println!("Your guess: {guess}");

    
        match guess.cmp(&secret_number) {
            Ordering::Greater=>println!("Too big"),
            Ordering::Less=>println!("Too little"),
            Ordering::Equal=>{
                println!("You Won");
                break;
            }
        }
    }
    }