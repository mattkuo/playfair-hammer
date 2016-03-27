extern crate rand;
use rand::distributions::{IndependentSample, Range};
use rand::{Rng, ThreadRng};

pub type CipherKey = [[char; 5]; 5];

pub struct Playfair {
    key: CipherKey,
    range: Range<usize>
}

impl Playfair {

    pub fn new() -> Playfair {
        Playfair {
            key: [['A', 'B', 'C', 'D', 'E'],
                  ['F', 'G', 'H', 'I', 'K'],
                  ['L', 'M', 'N', 'O', 'P'],
                  ['Q', 'R', 'S', 'T', 'U'],
                  ['V', 'W', 'X', 'Y', 'Z']],
            range: Range::new(0, 5)
        }
    }

    pub fn decipher(&self, cipher: &str) -> String {
        let mut deciphered_text = String::with_capacity(cipher.len());

        let mut index = 0;
        while index < cipher.len() {
            let deciphered_digram = self.decipher_digram(&cipher[index..index + 2]);
            deciphered_text.push_str(deciphered_digram.as_str());
            index += 2;
        }

        return deciphered_text;
    }

    // TODO: Refactor random functions in swapping helpers
    pub fn rand_modify_key(&mut self, rng: &mut ThreadRng) {
        let prob = rng.gen_range(1, 51);
        match prob {
            1 => self.swap_cols(rng),
            2 => self.swap_rows(rng),
            3 => self.swap_axis(),
            4 => self.swap_top_bottom(),
            5 => self.swap_left_right(),
            _ => self.swap_letters(rng)
        };
    }

    pub fn get_key(&self) -> CipherKey {
        return self.key;
    }

    pub fn set_key(&mut self, new_key: CipherKey) {
        self.key = new_key;
    }

    fn swap_letters(&mut self, rng: &mut ThreadRng) {
        let index_a = (self.range.ind_sample(rng), self.range.ind_sample(rng));
        let mut index_b = (self.range.ind_sample(rng), self.range.ind_sample(rng));

        while index_b == index_a {
            index_b = (self.range.ind_sample(rng), self.range.ind_sample(rng));
        }

        let temp_a = self.key[index_a.0][index_a.1];
        self.key[index_a.0][index_a.1] = self.key[index_b.0][index_b.1];
        self.key[index_b.0][index_b.1] = temp_a;
    }

    fn swap_rows(&mut self, rng: &mut ThreadRng) {
        let row_a = self.range.ind_sample(rng);
        let mut row_b = self.range.ind_sample(rng);

        while row_b == row_a {
            row_b = self.range.ind_sample(rng);
        }

        for index in 0..self.key.len() {
            let temp_a = self.key[row_a][index];
            self.key[row_a][index] = self.key[row_b][index];
            self.key[row_b][index] = temp_a;
        }
    }

    fn swap_cols(&mut self, rng: &mut ThreadRng) {
        let col_a = self.range.ind_sample(rng);
        let mut col_b = self.range.ind_sample(rng);

        while col_b == col_a {
            col_b = self.range.ind_sample(rng);
        }

        for index in 0..self.key.len() {
            let temp_a = self.key[index][col_a];
            self.key[index][col_a] = self.key[index][col_b];
            self.key[index][col_b] = temp_a;
        }
    }

    fn swap_axis(&mut self) {
        for i in 0..self.key.len() - 1 {
            for j in i + 1..self.key.len() {
                let temp = self.key[i][j];
                self.key[i][j] = self.key[j][i];
                self.key[j][i] = temp;
            }
        }
    }

    fn swap_top_bottom(&mut self) {
        for i in 0..self.key.len() / 2 {
            let bot = self.key.len() - i - 1;
            for j in 0..self.key.len() {
                let temp = self.key[i][j];
                self.key[i][j] = self.key[bot][j];
                self.key[bot][j] = temp;
            }
        }
    }

    fn swap_left_right(&mut self) {
        for i in 0..self.key.len() / 2 {
            let right = self.key.len() - i - 1;
            for j in 0..self.key.len() {
                let temp = self.key[j][i];
                self.key[j][i] = self.key[j][right];
                self.key[j][right] = temp;
            }
        }
    }

    fn decipher_digram(&self, digram: &str) -> String {
        let key:CipherKey = self.key;
        let mut chars_iter = digram.chars();
        let (a_row, a_col) = self.get_letter_key_index(chars_iter.next().unwrap());
        let (b_row, b_col) = self.get_letter_key_index(chars_iter.next().unwrap());

        let mut digram = String::with_capacity(2);
        let last_index = key.len() - 1;

        if a_row == b_row {
            digram.push(if a_col == 0 { key[a_row][last_index] } else { key[a_row][a_col - 1]});
            digram.push(if b_col == 0 { key[b_row][last_index] } else { key[b_row][b_col - 1]});
        } else if a_col == b_col {
            digram.push(if a_row == 0 { key[last_index][a_col] } else { key[a_row - 1][a_col]});
            digram.push(if b_row == 0 { key[last_index][b_col] } else { key[b_row - 1][b_col]});
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
        let test_str = "sihthtdqcusy".to_uppercase();
        let deciphered = playfair.decipher(&test_str);
        assert_eq!(deciphered, "THISISATESTX");
    }
}
