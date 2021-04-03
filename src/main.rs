use std::io;
use std::io::prelude::*;
use std::process::Command;
//use crossbeam_utils::thread;
use std::time::Duration;
use std::thread;

fn main() {

    let stdin = io::stdin();

    //extract packages into an array
    let packages = stdin.lock().lines();
    let mut vecc  = Vec::new(); 

    for package in packages {
        let temp = package.expect("failed");
        vecc.push(temp);

    }
    
    
    for line in vecc {
        let child = thread::spawn(move || {
        //s.spawn(move |_| {
            //extract the names of pacakges to search for
            //let nline = line.expect("failed");
            let seline: Vec<&str> = line.split(" ").collect();
            
            //get name of package
            let package_name = seline[0];
            
            get_pkg_info(package_name)
        
        });
        child.join().unwrap();
            
    };
        
   
    
    println!("Exiting Application!");
}

fn get_pkg_info(package_name:&str) {
    //execute dnf info command
    let output = Command::new("dnf")
    .args(&["info",package_name])
    .output()
    .expect("failed to execute process");

    let output = String::from_utf8_lossy(&output.stdout).to_string();
    let split_str: Vec<&str> = output.split("\n").collect();

    for meta in split_str {
        let meta = String::from(meta);
        
        let extract = match meta.get(..11) {
            Some(meta) => meta,
            None => "",
        };
        if extract == "Description"{
            println!("====== Package: {} ====",package_name);
            println!("--{}\n",meta);
            break;    
        }
    }
}