use std::{time::Duration, *}; //lib def

mod account; //file def
mod operations;

#[macro_use] //macro def
mod r#macro;

fn main() {
    'test: loop {
        let user = account::create_account();

        println!("{}", user.join_date);

        wait!(5, 0);

        break 'test;
    }

    let mut input: String = String::new();
    io::stdin().read_line(&mut input);
}

fn quit_function() {}
