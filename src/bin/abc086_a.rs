use atcoder_with_creusot::abc086_a::*;
use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    }
    if solve(a, b) {
        println!("Even");
    } else {
        println!("Odd");
    }
}
