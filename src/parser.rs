use std::io;
use std::io::{BufRead, Error, ErrorKind};
use std::fs::File;

use std::collections::HashMap;

pub fn read_ngram(path: &str) -> Result<(), io::Error> {
    let file = try!(File::open(path));
    let reader = io::BufReader::new(&file);

    // Mapping of n-gram to log probability
    let mut map: HashMap<String, i64> = HashMap::new();
    let mut total = 0;

    for line in reader.lines() {
        let line: String = line.unwrap();
        let collect_text: Vec<&str> = line.split(' ').collect();
        let text: String = collect_text[1].into();
        let amount: i64 = match text.parse() {
            Ok(result) => result,
            Err(_) => return Err(Error::new(ErrorKind::InvalidData, "Not an int"))
        };

        total += amount;
        map.insert(collect_text[0].into(), amount);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_read_file() {
        match read_ngram("./ngrams/quadgrams.txt") {
            Ok(_) => println!("done"),
            Err(e) => println!("{}", e)
        };
    }
}
