use std::fs;

pub fn parse() {

    let content = fs::read_to_string("config.toml").expect("Expected file name");
    let attributes : Vec<&str> = content.trim().lines().collect();

    for i in attributes.iter() {
        if i.trim().as_bytes()[0] as char == '[' || i.trim().as_bytes()[0] as char == '#'  {
            continue;
        } 
        
        //emulating of parsing attributes from a config.toml 


    }

}