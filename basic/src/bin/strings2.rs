
/*  Some Functionality defined on Strings */

fn main(){
    let text = "the red fox and the lazy dog";
    let words = text.split_whitespace();

    // Either this
    let word_vec: Vec<&str> = words.clone().collect(); 
    println!("using collect(): {:?}", word_vec);
    // OR
    let mut word_vec = Vec::new();
    word_vec.extend(words);
    println!("using extend(): {:?}", word_vec);


    let stripped: String = text.chars()
            .filter(|ch| ! ch.is_whitespace()).collect();
    
    println!("{}", stripped);
    /*
    The `filter` method takes a closure, which is Rust-speak for lambdas or anonymous
     functions. Here the argument type is clear from the context,
      so the explicit rule is relaxed.
     */

}