use std::{time::Duration, *}; //lib def

mod account; //file def
mod io;

#[macro_use] //macro def
mod r#macro;

fn main() {
    'test: loop {
        let user = account::create_account();

        println!("{}", user.join_date);

        wait!(5, 0);

        break 'test;
    }
}

fn quit_function() {}
