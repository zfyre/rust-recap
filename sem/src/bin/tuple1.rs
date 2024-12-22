
/*  Tuples can contain different types -> main difference from arrays */

use std::iter::zip;

fn add_mul(a: f64, b: f64) -> (f64, f64) {
    (a + b, a * b)
}

fn main(){
    let t = add_mul(3.018, 4.401);
    // can debug print!
    println!("t {:?}", t);

    // can 'index' the values
    println!("add {}, mul {}", t.0, t.1);
    
    // can extract values like in python
    let (add, mul) = t;
    println!("add {}, mul {}", add, mul);

    let tuple = ("zfYre", 5, 'c');
    println!("tuple {:?}", tuple);

    // enumerate is just like the python enumerate
    for t in ["zero", "one", "two"].iter().enumerate() { // here t is a tuple
        print!("{} {}; ", t.0, t.1);
    }
    println!("");

    // `zip` combines two iterators in a single iterator of tuples containing values from both
    let names = ["zfYre", "surfer", "adda"];
    let nums = [53, 54, 55];
    for p in zip(names.iter(), nums.iter()) {
        print!("{} {}; ", p.0, p.1);
    }

}