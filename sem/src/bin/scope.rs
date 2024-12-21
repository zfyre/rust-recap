/* reference must not outlive the owner! */

fn main(){
    {
        let a = 10;
        let b = "zfYre";
        {
            let c = "hello".to_string();
            // a, b,and c are visible
        }
        // string c is dropped here
        // a and b are still visible
        for i in 0..a {
            let b = &b[1..];
            // original b is no longer visible - it is `shadowed`
            println!("b:{}", b);
        }
        println!("b:{}", b);
        // the slice b is dropped
        // i is _not_ visible!
    }
}