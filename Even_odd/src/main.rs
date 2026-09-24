use std::io;

fn main(){

    println!("Hey Buddy Enter the Num:");

    let mut input = String::new();

    io::stdin().read_line( &mut input).expect("Failed in input");

    let input: i32 = input.trim().parse().expect("Failed to parse");

    if input % 2 == 0  {
        println!("The Number is Even, {input}");
    }else {
        println!("The Number is Odd, {input}");
    }

    
}