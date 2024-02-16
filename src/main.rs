mod my_funcs;
mod other_funcs;

use crate::my_funcs::{add_five, sub_five};
use crate::other_funcs::minus_funcs::mul_five;

fn main() {
    let a: u32 = add_five(45);
    let b: u32 = sub_five(45);
    let c: u32 = mul_five(45);
    println!("a = {}", a);
    println!("b = {}", b);
    println!("c = {}", c);
}
