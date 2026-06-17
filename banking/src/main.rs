use rand::{RngExt};
use std::{env, fs::{self, File}, };
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Deserialize, Serialize)]
 struct Account {
    id:u32,
    account_holder:String,
    opening_balance:u32,
    type_savings:bool,
    type_checking:bool,
}


fn main(){

    println!("Operaions:");
    println!("1. Create accout");
    println!("2. View account");
    

    let user_account = create_account(String::from("SDK"), 10, true, false);
}

fn create_account(holder:String, balance:u32, savings:bool, checking:bool) -> Account{

    let test:Account = Account{
        id:generate_id(),
        account_holder:holder,
        opening_balance:balance,
        type_savings:savings,
        type_checking:checking, 
    };

    generate_file();
    test
}

fn generate_id() -> u32{ //change this 
    let mut rng = rand::rng();
    let n:u32 = rng.random_range(..=99999999);
    n
}

fn generate_file(){
    
    let file_path = env::current_dir().unwrap();
    let mut path_translated = file_path.as_os_str().to_str().unwrap().to_string();
    path_translated.push_str("\\Data.Json");
    let path_exists:bool = fs::exists(path_translated).unwrap();

    if path_exists{
        println!("Data already exists.")
    }else{
        let data = File::create_new("Data.Json");
        println!("Created file");
    }
   
}

fn write_data(a:Account){

    let file_path = env::current_dir().unwrap();
    let mut path_translated = file_path.as_os_str().to_str().unwrap().to_string();
    path_translated.push_str("\\Data.Json");
    let path_exists:bool = fs::exists(path_translated).unwrap();

    if !path_exists{ 
        let mut data = File::create_new("Data.Json");
        println!("Created file");

        //add writing logic here 6/17

       
        serde_json::

        

    }else{
        println!("Json already exists.");
    }
}


