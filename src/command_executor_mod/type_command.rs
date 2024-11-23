use std::{env, fs, process};
use std::collections::HashMap;

pub fn type_command(arguments:Vec<&str>,executables:&HashMap<String,String>){
    if arguments[0]=="echo" || arguments[0]=="exit" || arguments[0] == "pwd"{
        shell_builtin(arguments[0]);
        return
    }
    if executables.contains_key(arguments[0]){
        directory_command(arguments[0],executables[arguments[0]].as_str());
        return
    }
    non_existent(arguments[0]);
}



fn shell_builtin(argument:&str){
    println!("{} is a shell builtin",argument);
}

fn directory_command(command:&str,path:&str){
    println!("{} is {}",command,path);
}

fn non_existent(argument:&str){
    println!("{}: not found",argument);
}

