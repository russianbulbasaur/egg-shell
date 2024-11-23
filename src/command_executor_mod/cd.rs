use std::env;

pub fn cd(arguments:Vec<&str>){
    if arguments.len() == 0{
        println!("no argument supplied to cd");
        return;
    }
    let dir = String::from(env::var("HOME").unwrap());
    let mut directory = arguments[0].trim();
    if directory == "~" {
        directory = dir.as_str();
    }
    let result = env::set_current_dir(directory);
    match result {
        Ok(res) => {},
        Err(error) => println!("{}",error)
    }
}