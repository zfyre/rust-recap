
/*
    However we can specify that the lifetime of the reference is at least as long as that of
    the struct itself.
*/

#[derive(Debug)]
struct A <'a> {
    s: &'a str,
}

// fn makes_a() -> A<'static> {
//     let string = "I'm a little string".to_string();
//     A { s: &string }
// }
 
/*
    **Above will throw ERROR since**

    There is no way that this could safely work,
    because string will be dropped when the function ends,
    and no reference to string can outlast it.
*/

fn main(){
    let s = "I'm a little string".to_string();
    let a = A {
        s: &s,
    };
    println!("{:?}", a);
}