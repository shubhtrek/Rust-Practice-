use std::io;

use rand::RngExt;

fn main(){

    println!("Guess The Number!");

    let secrate_num = rand::rng().random_range(1..=100);
    

    let mut attempts = 0;

    loop{

        println!("Enter ur Guess:");
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to red input");

        let guess:u32 = guess.trim().parse().expect("Please Etr a Num:");

        attempts += 1;

        if guess < secrate_num {
            println!("Too Low!!")
        }else if guess > secrate_num {
            println!("Too High!")
        }else {
            println!("You Win!!");
            println!("You took {attempts} attempts.");
            break;
        }
        
    }
}