use std::{io, string};

use chrono::{DateTime, Local};
use colored::Colorize;

pub struct Account {
    pub username: String,
    pub join_date: DateTime<Local>,
    pub coins: u32,
    pub pickaxe: u32,
    pub backpack: u32,
}

pub fn create_account() -> Account {
    println!("{}", "Welcome to Idle Miner".color("blue"));
    println!("Enter your username:");

    let mut input: String = String::new();
    io::stdin().read_line(&mut input);

    let mut test_account = Account {
        username: input,
        join_date: Local::now(),
        coins: 0,
        pickaxe: 0,
        backpack: 0,
    };

    println!(
        "Thanks for playing, {} Your join date is {}",
        test_account.username,
        Local::now()
    );

    return test_account;
}

fn account_exists() -> bool {
    true
}
