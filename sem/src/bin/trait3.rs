/*
Example: iterator over floating-point range
*/
// Recall the informal definition of an iterator; it is an struct with a next method which may return Some-thing or None

struct FRange {
    val: f64,
    end: f64,
    incr: f64
}

fn range(x1: f64, x2: f64, skip: f64) -> FRange {
    FRange {
        val: x1,
        end: x2,
        incr: skip
    }
}

// Kind of Overriding the Iterator methods for `FRange` struct!! -> very powerful thing!!
impl Iterator for FRange { // Iterator is a trait
    type Item = f64; // `type` is used to define an alias for an existing type!!

    fn next(&mut self) -> Option<Self::Item> {
        let res = self.val;
        if (self.end - res) < 1e-6  {
            None
        } else {
            self.val += self.incr;
            Some(res)
        }
    }
}

fn main(){
    for x in range(0.0, 1.2, 0.02) {
        println!("{:.2} ", x);
    }
    // All of the default iterator methods are available, so we can collect these values into a vector, map them, and so forth.
    let v: Vec<f64> = range(0.0, 1.0, 0.1).map(|x| x.sin()).collect();
    println!("{:?}", v);


}