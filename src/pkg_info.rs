use std::process::Command;
use colored::*;

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
        let package_name = package_data[0];

        let exec = Command::new("dnf")
            .args(&["info",package_name])
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
            .expect("failed to execute process");
        
        (exec, package_name)
        
    } else {

        //extract the names of pacakges to search for
        let package_data: Vec<&str> = line.split("/").collect();
        
        //get name of package
        let package_name = package_data[0];

        let exec = Command::new("yay")
            .args(&["info",package_name])
            .output()
            .expect("failed to execute process");

        (exec, package_name)
    };

    let (execution_str, package_name) = output;
    
    let output = String::from_utf8_lossy(&execution_str.stdout).to_string();
    let split_str: Vec<&str> = output.split("\n").collect();

    for meta in split_str {
        let meta = String::from(meta);
        
        let extract = match meta.get(..11) {
            Some(meta) => meta,
            None => "",
        };
        if extract == "Description"{
            println!("====== Package: {} ======",package_name.color("blue"));
            println!("--{}",meta);

            //extract updates from rust
            let get_last_update = Command::new("rpm")
            .args(&["-q",package_name,"--last"])
            .output()
            .expect("failed to execute process");
            let last_update = String::from_utf8_lossy(&get_last_update.stdout).to_string();
            
            let split_str_2: Vec<&str> = last_update.splitn(2," ").collect();

            //handle error where index does not exist?
            let extract_2 = match split_str_2.last(){
                Some(split_str_2) => split_str_2,
                None => "",
            };

            println!("--My Last Update: {}\n",extract_2.trim());
            break;    
        }
    }
    String::from(package_name)
}