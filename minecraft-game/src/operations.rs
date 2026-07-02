use std::{collections::hash_map::Values, env, fmt::Display, fs::File, io, string};

use envie::*;

pub fn get_data() -> Envie {
    Envie::load().expect("failed to load data.")
}

pub fn create_file() -> Result<(), io::Error> {
    let exe_path = env::current_dir()?;
    let save_path = exe_path.join(".env");
    let mut _file = File::create(save_path);
    Ok(())
}

pub fn write_data(key: &str, value: &str) {
    get_data().set(key, value);
}

pub fn return_value(key: &str) -> String {
    let value: String = get_data().get(&key).unwrap();
    value.to_string();
    value
}
