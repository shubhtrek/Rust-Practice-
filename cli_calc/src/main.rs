use std::io;

fn main() {
    println!("Enter first Num:");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let num: i32 = input.trim().parse().expect("Please entr valid number");

    println!("You Entered: {num}");

    println!("Entr operator (+, -, *, /:");

    let mut operator = String::new();

    io::stdin()
        .read_line(&mut operator)
        .expect("Failed to read operator");

    let operator = operator.trim();

    println!("Operator selected: {operator}");

    println!("Entr Second Num:");

    let mut sec_input = String::new();

    io::stdin()
        .read_line(&mut sec_input)
        .expect("Failed to read second Num");

    let sec_num: i32 = sec_input.trim().parse().expect("Please entr a valid Num");

    println!("Second Num: {sec_num}");

    match operator {
        "+" => {
            let result = num + sec_num;
            println!("Result: {result}");
        }

        "-" => {
            let result = num - sec_num;
            println!("Result: {result}");
        }

        "*" => {
            let result = num * sec_num;
            println!("Result: {result}");
        }

        "/" => {
            let result = num / sec_num;
            println!("Result: {result}");
        }
        _ => {
            println!("Invalid operator");
        }
    }
}
