
fn main(){

    // primitive data types are cheap to copy hence rust does this copying implicitly.
    let num = 125;
    let num2 = num;
    println!("num {}", num);

    // but for strings Rust will just simply 'move' the authority to 's2'...
    let s1 = "hello dolly".to_string();
    let s2 = s1; 
    // println!("s1 {}", s1); // This will throw error

    /*

    String
        | addr | ---------> Call me Ishmael.....
        | size |                    |
        | cap  |                    |
                                    |
    &str                            |
        | addr | -------------------|
        | size |

    f64
        | 8 bytes |
    
    for String copying we will need to allocate another block of heap memory and hence it's expensive to copy
    whereas the slice just refers to the address and the size of it hence, cheap to copy
    also the f64 is allocated on stack and cheap to copy.
    */
}