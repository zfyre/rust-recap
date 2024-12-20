
use std::env::args as ARGS;// for command-line arguments

// it returns an iterator over the arguments as strings, including the program name.
fn main(){
    // Without skipping the first string that is the program name
    // for arg in ARGS(){
    //     println!("'{}'", arg);
    // }   

    // skipping the program name
    let args: Vec<String> = ARGS().skip(1).collect();
    println!("{:?}", args);
}