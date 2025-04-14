use core::f64;
use std::io;

fn main() {

   let mut account_balance: f64 = 0.0;

   loop {

       match menu().trim() {
           "0" => break,
           "1" => {
               account_balance = deposit(account_balance);
               println!("Your current balance was updated to: {}\n", account_balance);
           },
           "2" => {
               println!("Your current balance is: {}\n", account_balance);
           },
           "3" => {
               account_balance = break_balance();
           },
           _ => println!("Invalid Option\n")
       }

   }

}

fn deposit(balance: f64) -> f64 {

    let mut value: String = String::new();

    println!("Please insert the ammount of value to be deposit: ");
    io::stdin()
    .read_line(&mut value)
    .expect("Failed to read line");

    let value: f64 = match value.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid value!\n");
            return balance
        }
    };

    return balance + value;

}

fn break_balance() -> f64 {
    println!("Your balance was reseted\n");
    return 0.0;
}

fn menu() -> String {

    print!("Welcome to yout digital bank. Select one option to continue: \n[1] - Deposit \n[2] - See Balance \n[3] - Break Balance \n[0] - Exit");

    let mut option = String::new();

    println!("\n");

    io::stdin()
    .read_line(&mut option)
    .expect("Failed to read line");

    return option;
}