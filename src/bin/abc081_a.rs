use creusot_contracts::trusted;
use proconio::marker::Bytes;
use proconio::{fastout, input};

#[trusted]
#[fastout]
fn main() {
    input! {
        s: Bytes,
    }
    println!("{}", solve::solve(s));
}

mod solve {
    use creusot_contracts::*;

    // "Counting i such that P(i) holds" doesn't seem to be in the Pearlite vocabulary
    // We have to encode such kind of logical function as a sum of 0s and 1s

    #[logic]
    fn marble_placed_01(s: Seq<u8>, i: Int) -> Int {
        // 49 is the ASCII code of '1'
        // We cannot use b'1' in Pearlite, even with @
        pearlite! {
            if s[i]@ == 49 {1} else {0}
        }
    }

    #[logic]
    #[requires(i >= 0)]
    #[variant(i)]
    fn marble_placed_01_partial_sum(s: Seq<u8>, i: Int) -> Int {
        pearlite! {
            if i == 0 {
                0
            } else {
                marble_placed_01_partial_sum(s, i - 1) + marble_placed_01(s, i - 1)
            }
        }
    }

    #[logic]
    fn marble_placed_01_sum(s: Seq<u8>) -> Int {
        pearlite! {
            marble_placed_01_partial_sum(s, s.len())
        }
    }

    #[requires(s@.len() == 3)]
    #[ensures(result@ == marble_placed_01_sum(s@))]
    pub fn solve(s: Vec<u8>) -> usize {
        let mut ans = 0;
        let mut i = 0;
        #[invariant(ans@ == marble_placed_01_partial_sum(s@, i@))]
        #[invariant(i@ <= s@.len())]
        while i < s.len() {
            i += 1;
            if s[i - 1] == b'1' {
                ans += 1;
            }
        }
        return ans;
    }
}
