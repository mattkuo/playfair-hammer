extern crate rand;
use rand::distributions::{IndependentSample, Range};
use rand::Rng;

pub type CipherKey = [[char; 5]; 5];

pub struct Playfair {
    key: CipherKey
}

impl Playfair {

    pub fn new() -> Playfair {
        Playfair {
            key: [['a', 'b', 'c', 'd', 'e'],
                  ['f', 'g', 'h', 'i', 'k'],
                  ['l', 'm', 'n', 'o', 'p'],
                  ['q', 'r', 's', 't', 'u'],
                  ['v', 'w', 'x', 'y', 'z']]
        }
    }

    pub fn decipher(&self, cipher: &str) -> String {
        let mut deciphered_text = String::new();

        let mut index = 0;
        while index < cipher.len() {
            let deciphered_digram = self.decipher_digram(&cipher[index..index + 2]);
            deciphered_text.push_str(deciphered_digram.as_str());
            index += 2;
        }

        return deciphered_text;
    }

    // TODO: Refactor random functions in swapping helpers
    pub fn rand_modify_key(&mut self) {
        let prob = rand::thread_rng().gen_range(0, 100);
        match prob {
            0..2 => self.swap_cols(),
            2..4 => self.swap_rows(),
            _ => self.swap_letters()
        };
    }

    pub fn get_key(&self) -> CipherKey {
        return self.key;
    }

    fn swap_letters(&mut self) {
        let between = Range::new(0, self.key.len());
        let mut rng = rand::thread_rng();

        let index_a = (between.ind_sample(&mut rng), between.ind_sample(&mut rng));
        let mut index_b = (between.ind_sample(&mut rng), between.ind_sample(&mut rng));

        while index_b == index_a {
            index_b = (between.ind_sample(&mut rng), between.ind_sample(&mut rng));
        }

        let temp_a = self.key[index_a.0][index_a.1];
        self.key[index_a.0][index_a.1] = self.key[index_b.0][index_b.1];
        self.key[index_b.0][index_b.1] = temp_a;
    }

    fn swap_rows(&mut self) {
        let between = Range::new(0, self.key.len());
        let mut rng = rand::thread_rng();

        let row_a = between.ind_sample(&mut rng);
        let mut row_b = between.ind_sample(&mut rng);

        while row_b == index_a {
            row_b = between.ind_sample(&mut rng);
        }

        for index in 0..self.key.len() {
            let temp_a = self.key[row_a][index];
            self.key[row_a][index] = self.key[row_b][index];
            self.key[row_b][index] = temp_a;
        }
    }

    fn swap_cols(&mut self) {
        let between = Range::new(0, self.key.len());
        let mut rng = rand::thread_rng();

        let col_a = between.ind_sample(&mut rng);
        let mut col_b = between.ind_sample(&mut rng);

        while col_b == col_a {
            col_b = between.ind_sample(&mut rng);
        }

        for index in 0..self.key.len() {
            let temp_a = self.key[index][col_a];
            self.key[index][col_a] = self.key[index][col_b];
            self.key[index][col_b] = temp_a;
        }
    }

    fn decipher_digram(&self, digram: &str) -> String {
        let key:CipherKey = self.key;
        let mut chars_iter = digram.chars();
        let (a_row, a_col) = self.get_letter_key_index(chars_iter.next().unwrap());
        let (b_row, b_col) = self.get_letter_key_index(chars_iter.next().unwrap());

        let mut digram = String::new();
        let last_index = key.len() - 1;

        if a_row == b_row {
            digram.push(if a_col == 0 { key[a_row][last_index] } else { key[a_row][a_col - 1]});
            digram.push(if b_col == 0 { key[b_row][last_index] } else { key[b_row][b_col - 1]});
        } else if a_col == b_col {
            digram.push(if a_row == 0 { key[last_index][a_col] } else { key[a_row - 1][a_col]});
            digram.push(if b_col == 0 { key[last_index][b_col] } else { key[b_row - 1][b_col]});
        } else {
            digram.push(key[a_row][b_col]);
            digram.push(key[b_row][a_col]);
        }

        return digram;
    }

    fn get_letter_key_index(&self, letter: char) -> (usize, usize) {
        let key:CipherKey = self.key;
        let (mut x, mut y) = (0, 0);

        'rows: for row in 0..key.len() {
            for col in 0..key[row].len() {
                if key[row][col] == letter {
                    x = row;
                    y = col;
                    break 'rows;
                }
            }
        }

        return (x, y);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decipher() {
        let playfair = Playfair::new();
        let test_str = "sihthtdqcusy";
        let deciphered = playfair.decipher(test_str);
        assert_eq!(deciphered, "thisisatestx");
    }
}
