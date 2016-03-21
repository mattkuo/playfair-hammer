pub type CipherKey = [[char; 5]; 5];

pub fn decipher(key: &CipherKey, cipher: &str) -> String {
    let mut deciphered_text = String::new();

    let mut index = 0;
    while index < cipher.len() {
        let deciphered_digram = decipher_digram(key, &cipher[index..index + 2]);
        deciphered_text.push_str(deciphered_digram.as_str());
        index += 2;
    }

    return deciphered_text;
}

fn decipher_digram(key: &CipherKey, digram: &str) -> String {
    let mut chars_iter = digram.chars();
    let (a_row, a_col) = get_index_of_letter(key, chars_iter.next().unwrap());
    let (b_row, b_col) = get_index_of_letter(key, chars_iter.next().unwrap());

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

fn get_index_of_letter(key: &CipherKey, letter: char) -> (usize, usize) {
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

#[cfg(test)]
mod tests {
    use super::*;
    use super::decipher_digram;

    static KEY: CipherKey = [['a', 'b', 'c', 'd', 'e'],
                            ['f', 'g', 'h', 'i', 'k'],
                            ['l', 'm', 'n', 'o', 'p'],
                            ['q', 'r', 's', 't', 'u'],
                            ['v', 'w', 'x', 'y', 'z']];

    #[test]
    fn test_decipher_digram() {
        let kl = decipher_digram(&KEY, "fp");
        assert_eq!(kl, "kl");

        let lk = decipher_digram(&KEY, "pf");
        assert_eq!(lk, "lk");

        let fg = decipher_digram(&KEY, "gh");
        assert_eq!(fg, "fg");
    }

    #[test]
    fn test_decipher() {
        let test_str = "sihthtdqcusy";
        let deciphered = decipher(&KEY, test_str);
        assert_eq!(deciphered, "thisisatestx");
    }
}
