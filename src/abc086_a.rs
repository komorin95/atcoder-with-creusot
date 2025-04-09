use creusot_contracts::*;

#[requires(a@ >= 1)]
#[requires(a@ <= 10000)]
#[requires(b@ >= 1)]
#[requires(b@ <= 10000)]
#[ensures(((a@ * b@) % 2 == 0) == result)]
pub fn solve1(a: usize, b: usize) -> bool {
    (a * b) % 2 == 0
}

#[requires(a@ >= 1)]
#[requires(a@ <= 10000)]
#[requires(b@ >= 1)]
#[requires(b@ <= 10000)]
#[ensures(((a@ * b@) % 2 == 0) == result)]
pub fn solve2(a: u16, b: u16) -> bool {
    a % 2 == 0 || b % 2 == 0
}
