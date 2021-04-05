use std::process::Command;

pub fn get_pkg_info(line: String) -> String {

    //extract the names of pacakges to search for
    //let nline = line.expect("failed");
    let seline: Vec<&str> = line.split(" ").collect();
    
    //get name of package
    let package_name = seline[0];

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
            println!("====== Package: {} ======",package_name);
            println!("--{}\n",meta);

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

            println!("--Last Update: {}",extract_2.trim());
            break;    
        }
    }
    String::from(package_name)
}