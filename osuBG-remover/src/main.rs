use std::{collections::btree_map::Entry, io};
use walkdir::WalkDir;



fn main(){

    println!("Please find your osu!(stable) song folder directory.\n");
    println!("Right click on it and copy as path.");
    println!("Please paste your path as is: ");

    let mut path = String::new();
    io::stdin().read_line(&mut path).unwrap().to_string().trim();
    let changed = path.replace("\"", "");
    
    
    remove_bg(changed);



}

fn remove_bg(s_folder_path:String){
    let mut subfolder_array: Vec<&str> = vec![];


    for entry in WalkDir::new(s_folder_path).max_depth(1){
        println!("{}", {entry});
    }
    

}