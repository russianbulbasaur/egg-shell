pub fn echo(arguments:Vec<&str>){
    for argument in arguments {
        print!("{} ", argument)
    }
    println!()
}