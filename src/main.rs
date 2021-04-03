use std::io;
use std::io::prelude::*;
use std::process::Command;


fn main() {
    let stdin = io::stdin();
    let mut current_key = String::from("");

    for line in stdin.lock().lines() {

        //let line = String::from(line.unwrap());
        let line = line.expect("failed");
        println!("====== Package:  {} ====",line);

        let output = Command::new("dnf")
        .args(&["info",&line])
        .output()
        .expect("failed to execute process");

        let output = String::from_utf8_lossy(&output.stdout).to_string();
        let split_str: Vec<&str> = output.split("\n").collect();
        //println!("{:?}",split_str);
        /*
        let second_str = match split_str.get(1){
            Some(second_str) => second_str,
            None => "",
        };   
        //println!("{:?}",split_str);

        if split_str[0].trim().is_empty() && !(second_str.is_empty()){
            println!("{}", second_str);
            current_key = String::from("");
        }
        else {
            current_key = String::from(split_str[0].trim());
        }
        */
        for meta in split_str {
            //println!("{}",meta);
            let meta = String::from(meta);
            
            let extract = match meta.get(..11) {
                Some(meta) => meta,
                None => "",
            };
            if extract == "Description"{
                println!("--{}",meta);    
            }
        }
        
        
        //println!("Here's the user input: {}", line);
    }
    println!("Exiting Application!");
}
