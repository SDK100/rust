use std::{io, string};

use chrono::{DateTime, Local};
use colored::Colorize;

pub struct Account {
    username: String,
    join_date: DateTime<Local>,
    coins: u32,
    pickaxe: u32,
    backpack: u32,
}

pub fn create_account() {
    println!("{}", "Welcome to Idle Miner".color("blue"));
    println!("Enter your username:");

    let mut input: String = String::new();
    io::stdin().read_line(&mut input);

    println!("Thanks for playing. Your join date is {}", Local::now());

    // let test = Account { username: input };
}

fn account_exists() -> bool {
    true
}
