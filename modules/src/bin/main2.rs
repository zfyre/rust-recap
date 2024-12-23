// AFTER EXPORTING THE FOO MODULE AS A LIB CRATE using 
/*
src$ rustc foo.rs --crate-type=lib
src$ ls -l libfoo.rlib
-rw-rw-r-- 1 steve steve 7888 Jan  5 13:35 libfoo.rlib
*/

extern crate foo;// even though it's showing error becaue rust-analyzer is not showing that we have linked the foo lib

fn main(){
    
}

/*
ran rustc main2.rs --extern foo=libfoo.rlib

-> The above code compiles successfully!! and gives us the main2.exe
*/