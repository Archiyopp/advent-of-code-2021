use solutions::first;
use solutions::second;
use solutions::third;

use solutions::leetcode;

pub mod solutions;
fn main() {
    first::first_half_puzzle();
    first::second_half_puzzle();
    second::puzzle();
    third::first_puzzle();
    println!(
        "{}",
        leetcode::Solution::roman_to_int("MCMXCIV".to_string())
    );
    println!("{}", leetcode::Solution::is_valid("[".to_string()));
}
