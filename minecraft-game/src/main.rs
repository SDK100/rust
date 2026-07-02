use std::{time::Duration, *};
mod account;
#[macro_use]
mod r#macro;

fn main() {
    'test: loop {
        account::create_account();

        thread::sleep(Duration::new(5, 0));

        println!("Waited for 5 seconds");
        wait!(5, 0);
        println!("Waited for 5 more seconds");

        break 'test;
    }
}

fn quit_function() {}
