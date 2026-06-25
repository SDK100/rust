use std::{fmt::format, os, time::SystemTime};

use chrono::{DateTime, Utc};


fn main(){

    let mut now = SystemTime::now();
    let translated:DateTime<Utc> = now.into();

    'test: loop {
        print!("{}", now);
        break 'test;
    }
}


fn quit_function(){


}