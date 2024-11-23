use std::env;

pub fn pwd(arguments:Vec<&str>){
    let pwd = env::current_dir().expect("cannot get pwd");
    println!("{}",pwd.into_os_string().into_string().unwrap());
}