
/*
Rust structs cannot inherit from other structs; they are all unique types.
There is no sub-typing. They are dumb data.

So how does one establish relationships between types? This is where traits come in.

*/

use std::fmt;


trait Show { // defined a trait
    fn show(&self) -> String;
}

impl Show for i32 {
    fn show(&self) -> String {
        format!("four-byte signed {}", self)
    }
}
impl Show for f64 {
    fn show(&self) -> String {
        format!("eight-byte float {}", self)
    }
}

// providing an implementation for the Person class for Debug

struct Person {
    first_name: String,
    second_name: String
}
impl Person {
    fn new(first: &str, second: &str) -> Person{
        
        let first = first.to_string();
        let second = second.to_string();
        Person {
            first_name: first, 
            second_name: second
        }
    }

    fn full_name(&self) -> String{
        format!("{} {}", self.first_name, self.second_name)
    }
}

impl fmt::Debug for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.full_name())
    }
}


fn main(){
    let answer = 42;
    let maybe_pi = 3.14;
    let s1 = answer.show();
    let s2 = maybe_pi.show();

    println!("show {}", s1);
    println!("show {}", s2);

    // ======= Implementation for Debug for Person using trait ========
    let p = Person::new("zfYre", "Messmer");
    println!("{:?}", p);
    
}