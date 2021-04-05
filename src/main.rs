mod pkg_info;
use pkg_info::get_pkg_info;

use std::io;
use std::io::prelude::*;
//use std::thread;

#[tokio::main]
async fn main() {

    let stdin = io::stdin();

    //extract packages into an array
    let packages = stdin.lock().lines();
    let mut vecc  = Vec::new(); 

    for package in packages {
        let temp = package.expect("failed");
        vecc.push(temp);

    }
    
    for line in vecc {
        let handle = tokio::spawn(async move {
        //s.spawn(move |_| {
            get_pkg_info(line)
        
        });
        handle.await.unwrap();
        
    };
    
    /* 
    vecc.into_iter()
        .map(|line| {
            //let seline: Vec<&str> = line.split(" ").collect();
            tokio::spawn(async move{
                let seline: Vec<&str> = line.split(" ").collect();
                println!("Result: {}",get_pkg_info(seline[0]));
            });
            
        });
    */
    
    println!("Exiting Application!");
}

