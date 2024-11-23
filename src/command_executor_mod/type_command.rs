use std::{env, fs, process};
use std::collections::HashMap;
pub fn type_command(arguments:Vec<&str>){
    let mut program_to_directory_map:HashMap<String,String> = HashMap::new();
    span_directories(&mut program_to_directory_map);
    if program_to_directory_map.contains_key(arguments[0]){
        directory_command(arguments[0],program_to_directory_map[arguments[0]].as_str());
        return
    }
    match arguments[0] {
        "echo" | "exit" => shell_builtin(arguments[0]),
        _ => non_existent(arguments[0])
    }
}

fn span_directories(result:&mut HashMap<String,String>){
    let path = env::var("PATH").unwrap();
    let directories:Vec<&str> = path.split(":").collect();
    for directory in directories{
        let paths = fs::read_dir(directory).unwrap();
        for path in paths{
            let dir_entry = path.unwrap();
            let file_name = dir_entry.file_name().into_string().unwrap();
            result.insert(file_name,dir_entry.path().display().to_string());
        }
    }
}

fn shell_builtin(argument:&str){
    println!("{} is a shell builtin",argument);
}

fn directory_command(command:&str,path:&str){
    let output = process::Command::new("/bin/ankit").output().expect("");
    println!("{:?}",String::from_utf8(output.stdout).expect("sdf"));
    println!("{} is {}",command,path);
}

fn non_existent(argument:&str){
    println!("{}: not found",argument);
}

