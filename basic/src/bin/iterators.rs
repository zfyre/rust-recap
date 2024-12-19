
/*
ITERATORS

An iterator is easy to define informally. It is an 'object' with a next method which returns
an Option.
As long as that value is not None, we keep calling next.
*/

fn main(){
    let mut itr = 0..3;
    assert_eq!(itr.next(), Some(0));
    assert_eq!(itr.next(), Some(1));
    assert_eq!(itr.next(), Some(2));
    assert_eq!(itr.next(), None);

    let arr = [10, 20, 30, 40, 50];
    for i in arr{
        println!("{}", i);
    }
    for i in arr.iter(){
        println!("{}", *i);
        println!("{}", i);
    }
    let slice = &arr;
    for i in slice {
        println!("{}", i);
    }

    let sum: i32  = (0..5).sum();
    println!("sum was {}", sum);

    let sum: i64 = [10, 20, 30].iter().sum();
    println!("sum was {}", sum);

    for s in slice.windows(2) { // windows 
        println!("window {:?}", s);
    }

    for s in slice.chunks(2) { // divides the slice into chunks of 2
        println!("chunks {:?}", s);
    }
}