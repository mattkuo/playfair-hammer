use std::io;
use std::io::BufRead;
use std::fs::File;

pub fn read_ngram(path: &str) -> Result<(), io::Error> {
    let file = try!(File::open(path));
    let reader = io::BufReader::new(&file);

    for line in reader.lines() {
        let text = line.unwrap().split(' ').collect();
        println!("{}", l);
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
