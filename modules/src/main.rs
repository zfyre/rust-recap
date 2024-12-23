
mod foo; // Now rustc main.rs will cause foo.rs to be compiled as well. There is no need to fool around with makefiles!
mod boo; // The compiler will also look at MODNAME/mod.rs, so this will work if I create a directory boo containing a file mod.rs


fn main() {
    let f = foo::Foo::new("hello world!");
    println!("{:?}", f);

    println!("{:?}", boo::answer());
    println!("{:?}", boo::bar::question());

    println!("{:?}", boo::bar_defined_as_a_file::question());

}

/*
================= IMPORTANT ================
An important point to note is there is no separate compilation here.
The main program and its module files will be recompiled each time.
 Larger programs will take a fair amount of time to build, although rustc is getting better at incremental compilation.
*/