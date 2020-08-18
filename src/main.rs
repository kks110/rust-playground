extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    loop {
        println!("Welcome, what would you like to do?:");
        println!("1) Guessing Game");
        println!("2) Convert temperatures");
        println!("3) Fibonacci");

        let option = get_user_input();

        if option == "1" {
            guessing_game();
            break
        } else if option == "2" {
            temp_converter();
            break
        } else if option == "3" {
            fibonacci();
            break
        } else {
            println!("Not a valid option, please try again.");
            continue
        }
    }
}

fn get_user_input() -> String {
    let mut input = String::new();
    let input: &str = loop {
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                break input.trim();
            }
            Err(_) => {
                println!("Failed to read line");
                continue
            },
        }
    };
    input.to_string()
}

fn get_user_input_float() -> f32 {
    let input = loop {
        match get_user_input().trim().parse() {
            Ok(num) => break num,
            Err(_) => {
                println!("Please enter a number");
                continue
            },
        };
    };
    input
}

fn get_user_input_unumber() -> u32 {
    let input = loop {
        match get_user_input().trim().parse() {
            Ok(num) => break num,
            Err(_) => {
                println!("Please enter a number");
                continue
            },
        };
    };
    input
}

fn temp_converter() {
    loop {
        println!("Welcome, what would you like to do?:");
        println!("1) Convert to Fahrenheit ");
        println!("2) Convert to Celsius");


        let option = get_user_input();

        if option != "1" && option != "2" {
            println!("Not a valid option, please try again.");
            continue
        };

        println!("What temp would you like to convert?: ");
        let temperature: f32 = get_user_input_float();

        if option == "1" {
            println!("Your temp in fahrenheit is {}", convert_to_fahrenheit(temperature));
            break
        } else if option == "2" {
            println!("Your temp in celsius is {}", convert_to_celsius(temperature));
            break
        }
    }
}

fn convert_to_fahrenheit(temp: f32) -> f32 {
    let multiplicative: f32 = 9.0/5.0;
    (temp * multiplicative) + 32.0
}

fn convert_to_celsius(temp: f32) -> f32 {
    let multiplicative: f32 = 5.0/9.0;
    (temp - 32.0) * (multiplicative)
}


fn guessing_game() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        let guess: u32 = get_user_input_unumber();

        println!("You guessed {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You Win");
                break;
            },
        }
    }
}

fn fibonacci() {
    println!("How many Fibonacci numbers would you like to see?");
    let nth = get_user_input_unumber();
    let mut previous: u128 = 0;
    let mut current: u128 = 1;
    let mut next: u128;
    for i in 0..nth {
        println!("{}: {}", i+1, current);
        next = previous + current;
        previous = current.clone();
        current = next;
    }
}