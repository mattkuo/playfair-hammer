use std::io;
use std::io::{BufRead, Error, ErrorKind};
use std::fs::File;

use std::collections::HashMap;

pub fn read_ngram(path: &str) -> Result<HashMap<String, f64>, io::Error> {
    let file = try!(File::open(path));
    let reader = io::BufReader::new(&file);

    // Mapping of n-gram to log probability
    let mut map: HashMap<String, f64> = HashMap::new();
    let mut total: f64 = 0;

    for line in reader.lines() {
        let line: String = line.unwrap();
        let collect_text: Vec<&str> = line.split(' ').collect();
        let text: String = collect_text[1].into();

        let amount: f64 = match text.parse() {
            Ok(result) => result,
            Err(_) => return Err(Error::new(ErrorKind::InvalidData, "Not an int"))
        };

        total += amount as f64;
        map.insert(collect_text[0].into(), amount as f64);
    }

    // Find log probability
    for (_, nums) in map.iter_mut() {
        *nums = (*nums / total).log10();
    }

    Ok(map)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_read_file() {
        let map: HashMap<String, f64> = match read_ngram("./ngrams/quadgrams.txt") {
            Ok(m) => m,
            Err(_) => HashMap::new()
        };

        let y: f64 = (13168375f64 / 4224127912f64).log10();

        assert_eq!(map.get("TION"), Some(&y) );
    }

}
