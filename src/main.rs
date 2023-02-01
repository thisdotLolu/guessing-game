use std::io;
use rand::Rng;//library for random number generation
use std::cmp::Ordering;//enum that is a resultof two things being compared
use colored::*;

fn main() {
    println!("Guess the number");

    let secret_number=rand::thread_rng().gen_range(1, 101);//for generating random number

    println!("The secret number is {}", secret_number);
    

    loop{
    println!("Please input your guess");

    let mut guess= String::new();//creates instance of empty string

    
    //get the users input
    io::stdin()
          .read_line(&mut guess)
          .expect("Failed to read line");//error handling  if user doesnt input appropriate input

    let guess: u32 = match guess.trim().parse(){
        Ok(num)=>num,
        Err(_)=>continue,
    };
    

    println!("You guessd :{}", guess);

    match guess.cmp(&secret_number){
        Ordering::Less=>println!("{}","Too small".red()),
        Ordering::Greater=>println!("{}","Too big!".red()),
        Ordering::Equal=>{
            println!("{}","You win".green());
            break;
        },
    }
}

    
}
