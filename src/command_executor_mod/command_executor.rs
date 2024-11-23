use std::collections::HashMap;
use std::{env, fs};
use crate::command_executor_mod::exit::exit;
use crate::command_executor_mod::echo::echo;
use crate::command_executor_mod::type_command::type_command;
use crate::command_executor_mod::pwd::pwd;
use crate::command_executor_mod::cd::cd;
pub struct CommandExecutor{
    executables:HashMap<String,String>
}

impl CommandExecutor{
    pub fn new_executor() -> CommandExecutor{
        let mut program_to_directory_map:HashMap<String,String> = HashMap::new();
        CommandExecutor::span_directories(&mut program_to_directory_map);
        return CommandExecutor{
            executables:program_to_directory_map
        }
    }

    pub fn execute(&self,command:&str) {
        let parsed_command = CommandExecutor::parse_command(command);
        match parsed_command.len(){
            0 => return,
            _ => CommandExecutor::execute_command_with_argument(parsed_command, &self.executables)
        }
    }


    fn execute_command_with_argument(command:Vec<&str>,executables:&HashMap<String,String>){
        match command[0] {
            "exit" => exit(Vec::from(&command[1..])),
            "echo" => echo(Vec::from(&command[1..])),
            "pwd" => pwd(Vec::from(&command[1..])),
            "cd" => cd(Vec::from(&command[1..])),
            "type" => type_command(Vec::from(&command[1..]),executables),
            _ => {
                if !executables.contains_key(command[0]) {
                    println!("{}: command not found",command[0]);
                    return
                }
                CommandExecutor::execute_executable(&command,executables[command[0]].to_string());
            }
        }
    }


    fn execute_executable(command:&Vec<&str>,executable:String){
        let arguments:Vec<String> = Vec::new();
        let output = std::process::Command::new(executable)
            .args(&command[1..])
            .output().expect("unable to execute");
        println!("{}",String::from_utf8(output.stdout).expect("Cannot convert output to string"))
    }

    fn parse_command(command:&str) -> Vec<&str>{
        let split:Vec<&str> = command.split(" ").collect::<Vec<&str>>();
        return split;
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
}

