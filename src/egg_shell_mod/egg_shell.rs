
use crate::command_executor_mod::command_executor::CommandExecutor;
use std::io;
use std::io::{Error, Write};

pub struct EggShell {
    command_executor:CommandExecutor
}

impl EggShell{
    pub fn new_shell() -> EggShell {
        return EggShell{
            command_executor : CommandExecutor::new_executor()
        }
    }

    pub fn run(&self) -> Result<String,Error> {
        let prompt:String = String::from("$ ");
        loop {
            let mut command:String = String::from("");
            print!("{}",prompt);
            io::stdout().flush()?;
            io::stdin().read_line(&mut command)?;
            let result = self.command_executor.execute(command.trim_matches('\n'));
            io::stdout().flush()?;
        }
    }
}