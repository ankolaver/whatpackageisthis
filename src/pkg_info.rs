use std::process::Command;
use colored::*;

use crate::dates::calculate_dates;

pub fn get_pkg_info(line: String) -> String {

    //Check Linux Distro Version
    let version = Command::new("cat")
        .args(&["/proc/version"])
        .output()
        .expect("failed to find Linux Version");

    let distro = String::from_utf8_lossy(&version.stdout).to_string();

    //execute dnf info command
    let output = if distro.contains("Red Hat") == true { 
        //extract the names of pacakges to search for
        let package_data: Vec<&str> = line.split(" ").collect();
        
        //get name of package
        let package_name = if package_data[0] == "Installed" { "Box2D.x86_64" } else {package_data[0]} ;

        let exec = Command::new("rpm")
            .args(&["-q",package_name,"-i"])
            .output()
            .expect("failed to execute process");
        
        (exec, package_name)

    } else if distro.contains("ubuntu") == true {
        //extract the names of pacakges to search for
        let package_data: Vec<&str> = line.split("/").collect();
        
        //get name of package
        let package_name = package_data[0];
        
        let exec = Command::new("apt")
            .args(&["info",package_name])
            .output()
            .expect("failed to execute process for apt pkg manager");
        
        (exec, package_name)
        
    } else {

        //extract the names of pacakges to search for
        let package_data: Vec<&str> = line.split("/").collect();
        
        //get name of package
        let package_name = package_data[0];

        let exec = Command::new("yay")
            .args(&["info",package_name])
            .output()
            .expect("Package Manager not available");

        (exec, package_name)
    };

    let (execution_str, package_name) = output;
    
    let output = String::from_utf8_lossy(&execution_str.stdout).to_string();

    //Get description
    let split_descript: Vec<&str> = output.split("Description :").collect();
    
    let pkg_descript = match split_descript.last() {
        Some(meta) => meta,
        None => "",
    };
    println!("====== Package: {} ======",package_name.truecolor(135,206,250));
    println!("{}: {} ","Description".color("green"),pkg_descript.trim());

    let (build_date, diff ) = calculate_dates(output, "Build Date", "Build Host");
    println!("{}{} ==> {} days since upstream dnf repo update","Last build date: ".truecolor(183,150,14),build_date, diff.num_days().to_string().truecolor(219,78,17));
    
    let (install_date, install_diff ) = calculate_dates(String::from_utf8_lossy(&execution_str.stdout).to_string(), "Install Date:", "Group");
    println!("{}{} ==> {} days since you updated","Last install date: ".truecolor(255,127,80),install_date, install_diff.num_days().to_string().truecolor(219,78,17));
    
    let output = String::from_utf8_lossy(&execution_str.stdout).to_string();
    let start_url = output.find("URL         :").unwrap_or(0);
    let end_url = output.find("Bug URL").unwrap_or(output.len());
    let repo_url = &output[start_url+14..end_url].trim().to_string();
    
    let output = String::from_utf8_lossy(&execution_str.stdout).to_string();
    let start = output.find("Bug URL     :").unwrap_or(0);
    let end = output.find("Summary").unwrap_or(output.len());
    
    let second_url = &mut output[start+14..end].trim().to_string();

    println!("{:?} {}", repo_url, second_url);

    if &repo_url.len() > &20 {
        if &repo_url[..19] == "https://github.com/" {
            return String::from(&repo_url[..])
        }
        else {
            return String::from("")
        }
    }
    else {
        return String::from("")
    }
}
