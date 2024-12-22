
#[derive(Debug)] // Directive
struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    fn new(first: &str, second: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: second.to_string()
        }
    }

    // Referencing the self value here, i.e not copying the self here!!
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    // we can substitute `Person` for `Self` 
    fn copy(&self) -> Self {
        Self::new(&self.first_name, &self.last_name)
    }

    // Methods may allow the data to be modified using a mutable self argument:
    fn set_first_name(&mut self, name: &str) {
        self.first_name = name.to_string();
    }

    // And the data will `move` into the method when a plain self argument is used:
    fn to_tuple(self) -> (String, String) { // the data will move into the tuple!!!
        (self.first_name, self.last_name)
    }

    // // if we do as following!!
    // fn to_tuple_too(&self) -> (String, String) {
    //     (self.first_name, self.last_name)
    // } // because the String cannot be copied implicitely

}

fn main(){
    let p = Person {
        first_name: "John".to_string(),
        last_name: "Cena".to_string()
    };

    println!("person {} {}", p.first_name, p.last_name);
    /*
    The values of a struct will be placed next to each other in memory,
    although you should not assume any particular memory layout,
    since the compiler will organize the memory for efficiency,
    not size,
    and there may be padding.
    */

    let p = Person::new("zfYre", "Messmer");
    println!("person {} {}", p.first_name, p.last_name);
    println!("fullname {}", p.full_name());

    println!("Using the derive directive -> {:?}", p);

    // converting the person to tuple
    let t = p.to_tuple();
    // println!("{}", p); ERROR: Because the value will be moved tos t
    println!("p converted to tuple {:?}", t);   


    /* SUMMARY
    &self argument: can use the values of the struct, but not change them
    &mut self argument: can modify the values
    self argument: will consume the value, which will move.
     */

}