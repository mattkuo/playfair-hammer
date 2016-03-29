extern crate rand;
mod scoring;
mod playfair;

use std::collections::HashMap;
use std::env;
use std::io::{self};
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use rand::{thread_rng, Rng};

use playfair::{Playfair};

const DELTA_TEMP: f64 = 1.0f64;
const CYCLE_SIZE: u32 = 50_000;

fn get_cipher() -> Result<String, &'static str>  {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        let mut stderr = std::io::stderr();
        writeln!(&mut stderr, "usage: {:?} [cipher_file]", args[0]).unwrap();
        return Err("Invalid number of arguments");
    }

    let mut cipher = String::new();
    if args.len() == 2 {
        let path = Path::new(&args[1]);
        let mut file = match File::open(&path) {
            Ok(f) => f,
            Err(_) => return Err("Could not open cipher file")
        };

        match file.read_to_string(&mut cipher) {
            Ok(_) => return Ok(cipher),
            Err(_) => return Err("Failed to read cipher file")
        };
    } else {
        match io::stdin().read_line(&mut cipher) {
            Ok(_) => return Ok(cipher),
            Err(_) => return Err("Failed to read cipher")
        };
    }
}

fn main() {

    let mut stderr = std::io::stderr();
    let map: HashMap<String, f64> = match scoring::read_ngram("./ngrams/quadgrams.txt") {
        Ok(m) => m,
        Err(msg) => {
            writeln!(&mut stderr, "Error: {}", msg).unwrap();
            return;
        }
    };

    let cipher = match get_cipher() {
        Ok(result) => result,
        Err(msg) => {
            writeln!(&mut stderr, "Error: {}", msg).unwrap();
            return;
        }
    };

    let cipher = cipher.trim().to_uppercase().replace(" ", "").replace("\n", "");

    if cipher.len() % 2 != 0 {
        writeln!(&mut stderr, "Error: Invalid ciphertext").unwrap();
        return;
    }

    let mut playfair = Playfair::new();
    let mut deciphered = playfair.decipher(&cipher);

    let mut best_fitness = scoring::get_text_score(&deciphered, 4, &map);
    let mut local_best_fitness = best_fitness;

    let mut best_key = playfair.get_key();
    let mut local_best_key = best_key;

    let mut temp = (10.0 + 0.087 * (cipher.len() as f64 - 84f64)).ceil();
    let mut rng = thread_rng();

    while temp > 0.0 {
        let mut counter = 0;
        let mut less_fit_count = 0;
        while counter < CYCLE_SIZE {
            playfair.rand_modify_key(&mut rng);
            deciphered = playfair.decipher(&cipher);
            let current_fitness = scoring::get_text_score(&deciphered, 4, &map);
            let delta_fitness = current_fitness - local_best_fitness;

            if delta_fitness >= 0.0 {
                local_best_fitness = current_fitness;
                local_best_key = playfair.get_key();
            } else {
                let prob = (delta_fitness / temp).exp();
                let rng_prob = rng.next_f64();
                if prob > rng_prob {
                    local_best_fitness = current_fitness;
                    local_best_key = playfair.get_key();
                    less_fit_count += 1;
                } else {
                    playfair.set_key(local_best_key);
                    counter += 1;
                }
            }

            if local_best_fitness > best_fitness {
                best_fitness = local_best_fitness;
                best_key = local_best_key;
                println!("Current temperature: {:?}", temp);
                println!("Best fitness: {:?}", best_fitness);
                println!("Best key: {:?}", best_key);
                println!("Deciphered: {:?}\n", deciphered);
            }
        }

        temp -= DELTA_TEMP;
        println!("Cooling to {:?}. Less fit children chosen: {}", temp, less_fit_count);
    }

}
