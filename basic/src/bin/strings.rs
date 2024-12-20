
fn dump(s: &str){
    println!("s: {}", s);
}
fn trace(arr: &[i32])->String{
    let mut res = '['.to_string();
    for x in arr{
        res += &x.to_string(); // += is defined on string slice not String hence the '&'.
        res += ", ";
    }
    res += "]";
    return res;
}
fn main(){
    // There are 2 types of strings like char* and std::string in C++
    // Here we have &str and String types
    // In fact, &str and String have a very similar relationship to each other as do &[T] to Vec<T>.

    let text = "hello dolby"; // The string slice of type &str
    let mut s = text.to_string(); // it's now an allocated string 

    dump(&text);
    dump(&s);
    /*
        Under the hood, String is basically a Vec<u8> and &str is &[u8],
        but those bytes must represent valid UTF-8 text.
    */
    // text.push('K'); // error: because of type &str not dynamic
    s.push('K');
    s.push_str(" is a good");
    s += " company";
    dump(&s);

    // Defining a function on array taht returns string
    let arr = [1, 2, 3, 4, 5];
    let res = format!("arr: {}",trace(&arr));
    println!("{}", res);

    // USING AS SLICES
    let text = "static";
    let string = "dynamic".to_string();

    let text_slice = &text[1..3]; // "ta"
    let string_slice = &string[1..3]; // "yn"
    println!("slices: {}, {}", text_slice, string_slice);

    // NOTE: SLICES CANNOT BE INDEXED
    // This is because they use the One True Encoding, UTF-8,
    // where a 'character' may be a number of bytes

    // use something like as follows for indexing
    let multilingual = "Hi! ¡Hola! привет!";
    for ch in multilingual.chars() {
        print!("{}", ch);
    }print!("\n");
    // IMP
    println!("len = Number of BYTES: {}", multilingual.len());
    println!("count = NUMBER of CHARS: {}", multilingual.chars().count());

    // now checking slicing using the iterators returned from find() function
    let maybe = multilingual.find('п');
    if maybe.is_some(){
        let hi = &multilingual[maybe.unwrap()..]; // returns the slice after the occurence of 'п'.
        println!("Russina 'hi': {}", hi);
    }
}