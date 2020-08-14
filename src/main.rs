extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Welcome, what would you like to do?:");
    println!("1) Guessing Game");
    println!("2) Convert temperatures");

    let option = get_user_input();

    if option == "1" {
        guessing_game()
    } else if option == "2" {
        temp_converter()
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

fn temp_converter() {
    println!("Welcome, what would you like to do?:");
    println!("1) Convert to Fahrenheit ");
    println!("2) Convert to Celsius");

    let option = get_user_input();

    println!("What temp would you like to convert?: ");
    let temp = get_user_input();

    let temp: f32 = loop {
        match temp.trim().parse() {
            Ok(num) => break num,
            Err(_) => {
                println!("Please enter a number");
                continue
            },
        }
    };

    if option == "1" {
        println!("Your temp in fahrenheit is {}", convert_to_fahrenheit(temp));
    } else if option == "2" {
        println!("Your temp in celsius is {}", convert_to_celsius(temp));
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

        let guess = get_user_input();

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number");
                continue
            },
        };

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