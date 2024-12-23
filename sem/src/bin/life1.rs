/*
    Rust will not allow a reference to be stored
    without knowing its lifetime
*/


#[derive(Debug)]
struct A {
    // s: &str, // will throw ERROR because rust dont know till when the s will be available to memory
    s: &'static str,
}

impl A {
    // Using the lifetime in functions
    fn how(i: u32) -> &'static str { // again we are returning a reference!! hence we need to specify the lifetime
        match i {
            0 => "none",
            1 => "one",
            _ => "many",
        }
    }
}
fn main(){
    let a = A {
        s: "hello zfYre!!" // A Sring literal -> String literals exists for the duration of the whole program
    };
    println!("{:?}", a);

    println!("{}", A::how(5)) // `how` here is an associated function wrt the class not it's object beacuse self is not present
    

}