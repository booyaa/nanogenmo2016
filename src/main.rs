extern crate numeral;

use numeral::Numeral;

const WORD_COUNT: u64 = 1666;

fn main() {
    let n = 1;
    for i in 1..WORD_COUNT {
        print!("{} ", n.ordinal());
    }
    println!("");
}
