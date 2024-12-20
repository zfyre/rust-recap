

fn main(){
    let multilingual = "the name of the fox is zfyre";

    // match automatically unwarps the value from Some.
    match multilingual.find('z') {
        Some(idx)=>{
            let name = &multilingual[idx..];
            println!("captured! {}", name);
        },
        None => println!("could'nt find the fox!")
    };
    
    // But if you're not interested in failure here, then if let is your friend:
    if let Some(idx) = multilingual.find('z'){
        println!("captured! {}", &multilingual[idx..]);
    }

    // match can act like switch cases in C and also return value like other constructs in Rust
    
    let n = 6;
    let text = match n {
        0 => "zero",
        1 => "one",
        2 => "two",
        3 => "three",
        // i32::MIN..=-1_i32 | 4_i32..=i32::MAX => todo!() // a..b => exclusive range and a..=b => inclusive range
        i32::MIN..=-1_i32 | 4_i32..=i32::MAX => "OOR" // a..b => exclusive range and a..=b => inclusive range
    };
    println!("the number is {}", text);

    let text = match n {
        0..=3 => "small",
        4..6 => "medium",
        _ => "large"
    };
    println!("The number {} is {}", n, text);

}