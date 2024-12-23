mod foo { // Now the following structs or functions or types can be referenced using foo::
    #[derive(Debug)]
    pub struct Foo {
        pub s: &'static str, // Here we have to specifiy the lifetime because we are assigning a reference.
    }

    // it's often recommended to hide these implementations and just make the fields public
    impl Foo{
        pub fn new(s: &'static str) -> Foo{
            Foo { 
                s: s 
            }
        }
    }
}

/*
====================== Info ========================
The set of functions and types `exported` from a module is called its `interface`.
`exported` means the public fields which are visible to the outside world.
*/


fn main() {
    let f = foo::Foo{
        s: "hello",
    };
    println!("{:?}", f);

    let f2 = foo::Foo::new("zfYre");
    println!("{:?}", f2);
}




/*
============= UPCOMING => BREAKING THE CODE INTO VARIOUS FILES ===================
*/