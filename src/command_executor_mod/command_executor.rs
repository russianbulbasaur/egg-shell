use crate::command_executor_mod::exit::exit;
use crate::command_executor_mod::echo::echo;
use crate::command_executor_mod::type_command::type_command;

pub struct CommandExecutor{
}

impl CommandExecutor{
    pub fn new_executor() -> CommandExecutor{
        return CommandExecutor{}
    }

    pub fn execute(&self,command:&str) {
        let parsed_command = CommandExecutor::parse_command(command);
        match parsed_command.len(){
            0 => return,
            _ => CommandExecutor::execute_command_with_argument(parsed_command)
        }
    }


    fn execute_command_with_argument(command:Vec<&str>){
        match command[0] {
            "exit" => exit(Vec::from(&command[1..])),
            "echo" => echo(Vec::from(&command[1..])),
            "type" => type_command(Vec::from(&command[1..])),
            _ => println!("{}: command not found",command[0])
        }
    }

    fn parse_command(command:&str) -> Vec<&str>{
        let split:Vec<&str> = command.split(" ").collect::<Vec<&str>>();
        return split;
    }
}

