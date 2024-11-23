
pub fn type_command(arguments:Vec<&str>){
    match arguments[0] {
        "echo" | "exit" => shell_builtin(arguments[0]),
        _ => non_existent(arguments[0])
    }
}

fn shell_builtin(argument:&str){
    println!("{} is a shell builtin",argument);
}

fn non_existent(argument:&str){
    println!("{}: not found",argument);
}

