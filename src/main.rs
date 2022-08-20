use std::collections::HashSet;
use text_io::read;
mod lib;

include!(concat!(env!("OUT_DIR"), "/dictionary.rs"));

fn main() {
    let allowed: HashSet<String> =
        HashSet::from_iter(DICTIONARY.iter().map(|(word, _)| word.to_string()));

    loop {
        lib::play(&allowed);
        println!("Play again? (y/n)");

        let answer: String = read!("{}");

        if !(answer.as_str() == "y") {
            break;
        }
    }
}
