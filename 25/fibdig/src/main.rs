extern crate num_bigint;
extern crate num_traits;

use num_bigint::BigUint;
use num_traits::{Zero, One, ToPrimitive};
use std::mem::replace;

fn main() {
    for (f,i) in Fib::new() {
        println!("f({}), len({}), {}", i, ndigits(&f), &f);
        /*if ndigits(&f) >= 1000 {
            println!("f( {} ) = {}", i, f);
            break;
        }*/
    }
}

/// Number of base 10 digits of n.
fn ndigits(n : &BigUint) -> u64 {
 //    match n.to_f64() {
   //     Some(f) => (f.ln() / 10.0f64.ln() ).ceil() as u64,
      /*  None    =>*/ ((n.bits() as f64) / (10f64).log2()).ceil() as u64//,
    //}
}

struct Fib {
    a : BigUint,
    b : BigUint,
    index : u64,
}
impl Fib {
    pub fn new() -> Fib {
        Fib { a : One::one(), b : Zero::zero(), index : 1 }
    }
}
impl Iterator for Fib {
    type Item = (BigUint, u64);
    fn next(&mut self) -> Option<Self::Item> {
        let curr = &self.b + &self.a;
        self.b = replace(&mut self.a, curr);
        self.index += 1;
        Some((self.a.clone(), self.index))
    }
}
