use std::io;
use rand::Rng;
use std::cmp::Ordering;
use colored::*;

fn main() {
    println!("Guess the number!");

    let secret_number =rand::thread_rng().gen_range(1..101);

    println!("secret number is {}",secret_number); 

    

    loop
    {    

        println!("Please input the number");

        let mut guess = String::new();

    io::stdin()
    .read_line(&mut  guess)
    .expect("error while input");

    
     let guess : u32 = match guess.trim().parse()
    {
        Ok(num) => num,
        Err(_) =>continue,
    };

    println!("your guessed number is {}",guess);
        
      match guess.cmp(&secret_number)
    {
        Ordering::Less => println!("{}","your guess is lesser than the expected".red()),
        Ordering::Greater => println!("{}","your guess is greater than the expected".blue()),
        Ordering::Equal => {
            println!("{}","your guess is correct!!".green());
            break;
        }
    }
    }

    
}

