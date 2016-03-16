use std::io;
use std::io::{BufRead, Error, ErrorKind};
use std::fs::File;

use std::collections::HashMap;

const NONEXISTANT_NGRAM: &'static str = "!!!!";

pub fn read_ngram(path: &str) -> Result<HashMap<String, f64>, io::Error> {
    let file = try!(File::open(path));
    let reader = io::BufReader::new(&file);

    // Mapping of n-gram to log probability
    let mut map: HashMap<String, f64> = HashMap::new();
    let mut total: f64 = 0f64;

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

    map.insert(NONEXISTANT_NGRAM.into(), (0.01f64 / total).log10());

    Ok(map)
}

pub fn get_text_score(text: &str, n: u32, score_map: &HashMap<String, f64>) -> f64 {
    let mut total_score: f64 = 0f64;
    let char_count = text.chars().count() as u32;

    for index in 0..char_count - n + 1{
        let sub_str = &text[index as usize..(index + n) as usize];
        let score: f64 = match score_map.get(sub_str) {
            Some(sub_str_score) => *sub_str_score,
            None => *(score_map.get(NONEXISTANT_NGRAM).unwrap())
        };

        total_score += score;
    }

    return total_score;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    #[ignore]
    fn test_read_file() {
        let map: HashMap<String, f64> = match read_ngram("./ngrams/quadgrams.txt") {
            Ok(m) => m,
            Err(_) => HashMap::new()
        };

        let y: f64 = (13168375f64 / 4224127912f64).log10();

        assert_eq!(map.get("TION"), Some(&y) );
    }

    #[test]
    #[ignore]
    fn test_score() {
        let map: HashMap<String, f64> = match read_ngram("./ngrams/quadgrams.txt") {
            Ok(m) => m,
            Err(_) => HashMap::new()
        };

        let score: f64 = get_text_score("ATTACKTHEEASTWALLOFTHECASTLEATDAWN", 4, &map);
        let score2: f64 = get_text_score("FYYFHPYMJJFXYBFQQTKYMJHFXYQJFYIFBS", 4, &map);
        assert_eq!(score, -127.77224079273714f64);
        assert_eq!(score2, -302.3543701340869);
    }

}
