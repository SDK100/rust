use std::{fmt::format, os, thread, time::{Duration, SystemTime}};
use chrono::{DateTime, Utc};
mod account;
#[macro_use]
mod r#macro;



fn main(){

    let duration = Duration::new(5,0);

    let mut now = SystemTime::now();
    let translated:DateTime<Utc> = now.into();

    'test: loop {
        account::create_account();
        let mut x = 2;
        println!("hi");
        x = x -1;
        thread::sleep(Duration::new(5, 0));

        println!("Waited for 5 seconds");
        wait!(5,0);
        println!("Waited for 5 more seconds");

        break 'test;
    }
}


fn quit_function(){


}