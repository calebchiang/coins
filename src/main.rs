use std::io;
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    println!("Select a Coin:");
    println!("1. Penny");
    println!("2. Nickel");
    println!("3. Dime");
    println!("4. Quarter");

    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let coin: Coin = match input.trim().parse() {
        Ok(num) => {
            match num {
                1 => Coin::Penny,
                2 => Coin::Nickel,
                3 => Coin::Dime,
                4 => Coin::Quarter,
                _ => {
                    println!("Invalid selection, please enter a number from 1 to 4.");
                    return;
                }
            }
        },
        Err(_) => {
            println!("Please enter a valid number.");
            return;
        }
    };

    println!("The value of the selected coin is: {} cents", value_in_cents(coin));

}
