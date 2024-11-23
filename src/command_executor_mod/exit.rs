pub fn exit(arguments:Vec<&str>){
    match arguments[0] {
        "0" => std::process::exit(0),
        _ => println!("invalid code to exit")
    }
}