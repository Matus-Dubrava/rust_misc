pub mod lifetimes;

use crate::lifetimes::StrSplit;

fn main() {
    let input = "a b c d e";
    let letters = StrSplit::new(input, " ");
    for letter in letters {
        println!("{}", letter);
    }

    dbg!(input.find(" "));
    if let Some(delimiter) = input.find(" ") {
        dbg!(&input[..delimiter]);
    }
}
