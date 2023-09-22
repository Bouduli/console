///Will clear the console by printing control characters to the console. Likely cross platform - however only tested in bash terminal. 
pub fn clear() -> (){
    //Some black magic shii to clear the console
    print!("\x1B[2J\x1B[1;1H");
}
///A function to take input from stdin.
///A message is provided that will be used to prompt the terminal what to input. 
pub fn std_input(msg : &str)->String{
    use std::io::{self, Write};
    let mut input:String = String::new();

    print!("{msg}"); 
    let _ = io::stdout().flush();
    io::stdin().read_line(&mut input).expect("Unable to read input"); 

    //Implicitly returns the string from stdin trimmed. 
    input.trim().to_string()
}


    

///Awaits user input - a means to stay inside a loop iteration. 
pub fn await_continue() -> (){
    use std::io::{self};
    println!("Press [enter] to continue...\r");
    let mut input:String = String::new();
    io::stdin().read_line(&mut input);

}