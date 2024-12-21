

fn dump(s: String){ // Not by reference
    println!("Choice 2 ->s: {}", s);
}

fn dump1(s: &String){
    println!("Choice 1 ->s: {}", s);
}

fn better_dump(s: &str){ // string slice OR static string
    println!("BETTER DUMP -> S: {}", s);
}
fn main(){
    let s = "hello world!".to_string();
    // dump(s);
    // println!("s {}", s); // Same type of error becaue we have moved the String to teh function

    /* Two Choices
        - pass by reference
        - clone / copy the string
    */

    // choice 1
    dump1(&s);
    println!("s:{}", s);
    
    // choice 2
    dump(s.clone());
    println!("s:{}", s);


    // best choice
    better_dump("zfYre");
    better_dump(&"zfYre".to_string());
}   