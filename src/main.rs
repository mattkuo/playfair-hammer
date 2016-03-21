mod scoring;
mod playfair;

use std::collections::HashMap;

fn main() {
    let mut key = [['a', 'b', 'c', 'd', 'e'],
                    ['f', 'g', 'h', 'i', 'k'],
                    ['l', 'm', 'n', 'o', 'p'],
                    ['q', 'r', 's', 't', 'u'],
                    ['v', 'w', 'x', 'y', 'z']];

    let map: HashMap<String, f64> = match scoring::read_ngram("./ngrams/quadgrams.txt") {
        Ok(m) => m,
        Err(err) => panic!("Failed to parse ngram file: {:?}", err)
    };

}
