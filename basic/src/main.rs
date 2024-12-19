use std::f64::consts; // This is a use statement, it is like import in Python3

fn sqr(x: f64) -> f64 { // Here I have to specify the type of the input and output unlike in let answer = 42
    x*x
}// The value of the code block is the last statement in the block

// The () type is the empty type, nada, void, zilch, nothing. 

// A few more functions examples
fn abs(x: f64) -> f64 {
    if x > 0.0 {x} else {-x}
}
fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {min} else if x > max {max} else {x}
}
fn factorial(n: u64, m: u64)->u64{
    if n ==0 {1} else {(factorial(n-1, m) * (n)%m)%m} // if statement is always evaluated follwing with else.
}
fn by_ref(x: &i32) -> i32 {
    *x+1
}
fn by_mut_ref(x: &mut f64) {
    *x += 1.0;
}

fn main() {

    /* HELLO WORLD! */

    let answer = 42; // Variable 
    println!("Hello {}", answer); // printing like in Python3
    assert_eq!(answer, 42); // Asserting like in Python3

    // Looping & Conditions
    for i in 0..5{
        if i%2 == 0 {
            println!("even {}", i);
        } 
        else {
            println!("odd {}", i);
        }
    }

    // Value and Reference
    for i in 0..5{
        let even_odd = if i%2==0 {"even"} else {"odd"};
        println!("{} {}", even_odd, i);
    }

    // Mutability & traits
    let mut sum = 0.0; // A float value 
    for i in 0..5{
        sum += i as f32;           // Type casting needs to be done because rust is strongly typed
    }
    println!("Sum {}", sum);

    // Functions
    let x = 3.8;
    // let x = 2; // Panic! because x is integer and sqr expects float
    println!("Square of {} is {}", x, sqr(x));

    // More functions
    println!("Absolute value of {} is {}", -3.8, abs(-3.8));
    println!("Clamp of {} is {}", 3.8, clamp(3.8, 0.0, 1.0));
    println!("Factorial of {} is {}", 100, factorial(100, 10000007));

    // Pass by reference and mutable reference
    let i = 10;
    let res1 = by_ref(&i);
    let res2 = by_ref(&41);
    println!("{} {}", res1, res2);

    let mut res = 0.0;
    by_mut_ref(&mut res);
    println!("{}", res);

    /* LEARNING WHERE TO FIND ROPES */

    // Using the rust documentation
    let x: f64 = std::f64::consts::PI / 2.0; // Redeclaring the variable x
    let y = x.cos(); // sin is a method of f32 
    println!("cos({}) = {}", x, y);

    let abs_difference = ((consts::PI * 2.0).cos() - 1.0).abs();
    assert!(abs_difference < 1e-10); // cousin of assert_eq!, check expression is true

    // Using `use` statements 
    let x: f64 = consts::PI / 3.0;
    let y = x.cos();
    println!("cos({}) = {}", x, y);

}
