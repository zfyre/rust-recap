
use std::env;
use std::fs::File;
use std::io::Read;
use std::io; 

// Now useing the Result for reading the file without expecting any crashes
fn read_to_string(filename: &str) -> Result<String, io::Error> {
    let mut file = match File::open(&filename) {
        Ok(f) => f,
        Err(e) => return Err(e)
    };
    let mut text = String::new();
    match file.read_to_string(&mut text) { // no ;  hence returns the result
        Ok(_) => Ok(text),
        Err(e) => Err(e)
    }
}

fn read_to_string_small(filename: &str) -> io::Result<String> {
    let mut file = File::open(&filename)?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;
    Ok(text)
}

// OR -> Used in old code bases instead of `?`

// fn read_to_string(filename: &str) -> io::Result<String> {
//     let mut file = try!(File::open(&filename));
//     let mut text = String::new();
//     try!(file.read_to_string(&mut text));
//     Ok(text)
// }

fn main(){
    let filename = env::args().nth(1).expect("please supply as filename");

    let text = read_to_string(&filename);
    let text_small = read_to_string_small(&filename);
    println!("{:?}", text.unwrap_or("Some Error Occured while unwrapping".to_string()));
    println!("{:?}", text_small
                        .unwrap_or("Some Error Occured while unwrapping"
                        .to_string()
                    ).split_whitespace().count());

}