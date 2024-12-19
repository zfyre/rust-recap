
/*
Arrays are not used that often in Rust, because the type of an array includes its size.
The type of the array in the example is [i32; 4]; the type of [10, 20] would be [i32; 2] 
and so forth: they have different types.

So they are bastards to pass around as function arguments.
*/

// SLICES

/*
The relationship between Rust arrays and slices is similar to that between C arrays and
pointers, except for two important differences - 
Rust slices keep track of their size (and will panic if you try to access outside that size)
and you have to explicitly say that you want to pass an array as a slice using the & operator.
*/

fn sum(values: &[i32]) -> i32 { // We are passing a reference
    let mut res = 0;
    for i in 0..values.len(){
        res += values[i];
    }
    res
}

// A C programmer pronounces & as 'address of'; a Rust programmer pronounces it 'borrow'.

fn main(){
    println!("Hello, arrays!");

    let arr = [10 , 20, 30, 40, 50];
    let first = arr[0];
    println!("First {}", first);

    for i in 0..arr.len(){ // arr.len() returns the length of the array
        println!("[{}] = {}", i, arr[i]);
    }

    let res = sum(&arr);
    println!("Sum {}", res);

    // SLICING AND DICING

    let ints = [1, 2, 3];
    let floats = [1.1, 2.2, 3.3];
    let strings = ["hello", "world"];
    let arr_ints = [[1, 2], [3, 4]];
    // Debug print format for arrays
    println!("{:?}", ints);
    println!("{:?}", floats);
    println!("{:?}", strings);
    println!("{:?}", arr_ints);

    // Dicing: basically rust has combine `referencing` from C and `view` from Python
    let slice1 = &ints[0..2];
    let slice2 = &ints[1..];
    println!("{:?}", slice1);
    println!("{:?}", slice2);

    //  *** OPTION VALUES ***

    // Note: the slices are themselves indexable
    let arr = [0, 1, 2, 3];
    let slice = &arr[2..3];
    // let var: () = slice[0]; -> slice[0] is an i32 not a &i32
    // let var: () = slice.get(0);// -> slice.get(0) is an Option<&i32>
    println!("Slice val at index 0 is {:?}", slice[0]);

    // Since
    let var = slice.get(3); // index 3 doesnt exists
    // println!("{}", var.unwrap()); // This will panic

    if var.is_some(){println!("var = {}", var.unwrap());}
    else {println!("var is None");}

    // Another way to do the same thing
    match var {
        Some(val) => println!("var = {}", val),
        None => println!("var is None")
    }
    // OR
    println!("var = {}", *var.unwrap_or(&-1));// * because var is Option<&i32> and &-1 because we are dereferencing





}
