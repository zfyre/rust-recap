
// There is a very intimate relation between vectors and slices:

fn dump1(v: &Vec<i32>){
    println!("Dump 1: {:?}", v);
}
fn dump2(v: &[i32]){
    println!("Dump 2: {:?}", v);
}

fn main(){
    let mut v = Vec::new(); // Vectors must be intialized as mutable
    v.push(10);
    v.push(20);
    v.push(30);

    let first = v[0]; // panic id out of range
    let maybe_first = v.get(0); // returns an Option<&T>

    println!("{:?}", first);
    println!("{:?}", maybe_first);

    // Both dumps will work on vectors
    dump1(&v); 
    dump2(&v); 

    
    let slice = &v[1..];

    //But vec dump will not work on slices
    // dump1(slice); // panic
    dump2(slice);

    // A little marco for initializing
    let mut v = vec![984, 10, 49,5452, 20, 30, 30, 30];
    dump1(&v);

    // A few functions implemented on vectors
    v.sort()    ; // sorts in place
    dump1(&v);
    v.dedup(); // removes duplicates
    dump1(&v);

}