pub fn exit(arguments:Vec<&str>){
    if arguments.len() == 0 {
        println!("supply 0 as an argument to exit");
        return;
    }
    match arguments[0] {
        "0" => std::process::exit(0),
        _ => println!("invalid code to exit")
    }
}