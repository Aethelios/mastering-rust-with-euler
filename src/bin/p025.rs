use num_bigint::BigUint;
use num_traits::{Zero, One};
use std::mem::replace;


fn fib(n: usize) -> BigUint {
    let mut f0: BigUint = Zero::zero();
    let mut f1: BigUint = One::one();
    for _ in 0..n {
        let f2 = f0 + &f1;
        f0 = replace(&mut f1, f2);
    }
    f0
}
fn main(){
    let mut n = 1;
    while (fib(n).to_string()).len()<1000 {
        n+=1;
    }
    println!("n = {}", n);
}