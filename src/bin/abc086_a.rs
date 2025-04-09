use atcoder_with_creusot::abc086_a::*;
use std::io::{BufWriter, Read, Write, stdin, stdout};

fn main() {
    let mut stdin = stdin();
    let stdout = stdout();
    let mut output = BufWriter::new(stdout.lock());
    let mut input_str = String::new();
    let _ = stdin.read_to_string(&mut input_str);
    let mut input_iter = input_str.split_ascii_whitespace();
    macro_rules! read(() => (input_iter.next().unwrap().parse().unwrap()));
    macro_rules! print(( $( $x:expr ),*) => (write!(output, $($x,)*).unwrap()));
    macro_rules! println(( $( $x:expr ),*) => (writeln!(output, $($x,)*).unwrap()));

    let a = read!();
    let b = read!();
    if solve(a, b) {
        println!("Even");
    } else {
        println!("Odd");
    }
}
