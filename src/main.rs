mod pkg_info;
use pkg_info::get_pkg_info;

mod dates;

mod bugs_scraper;
use crate::bugs_scraper::scrape_version;
use crate::bugs_scraper::scrape_web;

use std::io;
use std::io::prelude::*;

#[tokio::main]
async fn main() {
    let stdin = io::stdin();

    //extract packages into an array
    let packages = stdin.lock().lines();
    let mut vecc = Vec::new();

    for package in packages {
        let temp = package.expect("failed");
        vecc.push(temp);
    }

    for line in vecc {
        let main_handle = tokio::spawn(async move {
            //let child = thread::spawn(move || {
            let (link, pkg_version, build_date) = get_pkg_info(line);
            
            println!(" ");
            //spawn web scraping for common issues
            if link != "" {
                let other_l = link.clone();

                //Extract Versions from github and compare time taken for maintainer
                // to build package for rpm purpose
                println!("❄️ Current Package Version {} ❄️", pkg_version);
                let handle2 = tokio::spawn(scrape_version(link, pkg_version, build_date));
                
                //Get common bugs from github
                let handle = tokio::spawn(scrape_web(other_l));

                //wait for future object running in async
                handle2.await.unwrap();
                //wait for future object running in async
                handle.await.unwrap();

                
            }
            println!(" ");
        });
        //child.join().unwrap();
        main_handle.await.unwrap();
    }
    println!("✅");
}
