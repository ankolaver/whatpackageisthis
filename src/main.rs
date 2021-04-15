mod pkg_info;
use pkg_info::get_pkg_info;

mod dates;

mod bugs_scraper;
use crate::bugs_scraper::scrape_web;

use std::io;
use std::io::prelude::*;
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
        //let child = thread::spawn(move || {            
            let link = get_pkg_info(line);
            
            //spawn web scraping for common issues
            if link != "" {
                let handle = tokio::spawn(scrape_web(link));
                
                //wait for future object running in async
                handle.await.unwrap();
            }
            
            println!(" ");
            
        });
        //child.join().unwrap();
        handle.await.unwrap();
    };
    
    //
    
    println!("✅");
}

