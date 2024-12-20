
use std::env;

fn main(){
    // here the first holds the string at index 1 not 0.
    let first = env::args().nth(1).expect("please supply an argument");
    let n: i32 = first.parse()
                    .expect("not an integer!");
    
    println!("The number is {:?}", n);
}