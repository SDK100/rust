use std::{array, vec};

use chrono::{DateTime, Local, Utc};
use colored::Colorize;
use sqlite::*;





pub struct Account{

    username:String,
    join_date:DateTime<Local>,
    coins:u32,
    pickaxe:u32,
}


pub fn create_account(){
    println!("{}","Welcome to Idle Miner".color("blue"));
    write_data();
    insert_existing_data("SDK100", 20, 15);
}

fn write_data(){

    let connection = sqlite::open("data.db").unwrap();
    let query = "
        CREATE TABLE IF NOT EXISTS data (username TEXT, coins INTEGER, pickaxe INTEGER);";
    connection.execute(query).unwrap();
}

fn insert_existing_data(d1:&str, d2:i64, d3:i64){
    let connection = sqlite::open("data.db").unwrap();
    let query = "
        INSERT INTO data (username, coins, pickaxe)
        VALUES (?,?,?)
        ON CONFLICT(username) DO UPDATE SET
            coins = excluded.coins,
            pickaxe = excluded.pickaxe;
    ";

    let mut statement = connection.prepare(query).unwrap();

    statement.bind((1, d1)).unwrap();
    statement.bind((2, d2)).unwrap();
    statement.bind((3, d3)).unwrap();

    statement.next().unwrap();
}
