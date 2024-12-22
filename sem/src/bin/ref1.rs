
/* A further Rust-specific issue is that a variable may appear to be
 in scope, but its value has moved */

fn main(){
    let s1 = "hello zfYre".to_string();
    let mut rs1 = &s1; // s1 slice
    {
        let tmp = "hello world".to_string();
        rs1 = &tmp; // shadowed to tmp slice

        // rs1 = &s1; // Adding this line will reassign the borrow and hence now no error!!
    }
    // println!("ref {}", rs1); // Will give an error!!

    // The scope of tmp is dropped while it was still borrowed by rs1!

    /* Here we see a classic problem of a pointer pointing to deleted memory 
        Rust here is to save us.
    */
    }