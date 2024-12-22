
/* WHAT ARE TYPE BOUNDS REALLY ?
*/

// fn sqr<T> (x: T) -> T {
//     x*x
// }

/* and obviously we get an eror and rust suggests the following:

    fn sqr<T: std::ops::Mul<Output = T>> (x: T) -> T {
*/

fn sqr<T> (x: T) -> T 
where T: std::ops::Mul<Output = T> + Copy{
    x*x
} // Applying two restrictions one that the `T` is multiplyable and the Copy exists for it.

fn main(){
     let res1 = sqr(10.0);
     let res2 = sqr(10);
     let res3 = sqr(10_f128);
     println!("res1 {}", res1);
     println!("res2 {}", res2);
     println!("res3 {:?}", res3);
}

/*
======================== NOTE =============================

It is a bit simpler in C++:

template <typename T>
T sqr(x: T) {
    return x * x;
}

but (to be honest) C++ is adopting cowboy tactics here. C++ template errors are famously bad,
because all the compiler knows (ultimately) is that some operator or method is not defined.
The C++ committee knows this is a problem and so they are working toward concepts,
which are pretty much like trait-constrained type parameters in Rust.

Rust generic functions may look a bit overwhelming at first,
but being explicit means you will know exactly what kind of values you can safely feed it,
just by looking at the definition.

These functions are called monomorphic, in constrast to polymorphic.
The body of the function is compiled separately for each unique type.
With polymorphic functions, the same machine code works with each matching type,
dynamically dispatching the correct method.

Monomorphic produces faster code, specialized for the particular type, and can often be inlined.
So when sqr(x) is seen, it's effectively replaced with x*x.
The downside is that large generic functions produce a lot of code, for each type used,
which can result in code bloat. As always, there are trade-offs;
an experienced person learns to make the right choice for the job.
*/