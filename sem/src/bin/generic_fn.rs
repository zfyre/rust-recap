

/*
We want a function which will dump out any value that implements Debug.
Here is a first attempt at a generic function, where we can pass a reference
to any type of value.
*/

// fn dump<T> (value: &T) {
    //     println!("value is {:?}",value);
// }
/* ===== However, Rust clearly knows nothing about this generic type T: ====== */


/* For this to work, Rust needs to be told that T does in fact implement Debug! */
fn dump<T> (value: &T) where T: std::fmt::Debug {
    println!("{:?}", value);    
}

/* IMPORTANT!!! */
// where T: std::fmt::Debug is called `type bounds` that is it lets rust know that this type is capable of this!!

fn dump_to<T> (value: &T, s: &mut String) where T: std::fmt::Debug {
    let to_dump = format!("{:?}", value);
    *s = to_dump.clone();
}

/* LOOK FOR THE FOLLOWING ERROR!! */
// fn dump_to_with_write<T> (value: &T, s: String) where T: std::fmt::Debug {
//  Ē   write!(s, "{:?}", value)
// }

#[derive(Debug)]
struct Foo {
    name: String
}

fn main(){
    let n = 42;
    dump(&n);

    let foo = Foo{
        name: "zfYre".to_string()
    };

    dump(&foo);

    let mut s = String::new();
    dump_to(&foo, &mut s);


}
