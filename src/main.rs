extern crate rand;

mod scoring;
mod playfair;

use std::collections::HashMap;
use rand::{thread_rng, Rng};
use playfair::{Playfair};

const DELTA_TEMP: f64 = 1.0f64;
const CYCLE_SIZE: u32 = 50_000;

fn main() {

    let map: HashMap<String, f64> = match scoring::read_ngram("./ngrams/quadgrams.txt") {
        Ok(m) => m,
        Err(err) => panic!("Failed to parse ngram file: {:?}", err)
    };

    let cipher = "LBINBCRBMOFWFDOFTKOCQUEOKOAWEORMFOTKOCREXVBTIVPLRBOIOANQEADMFTNQNROZFOFOFMHTCMREAEKMMEOMOCWAOTMFNVKOEOFOFOFMHTCMREOFMFCVQCBKMNRB";

    let mut playfair = Playfair::new();
    let mut deciphered = playfair.decipher(&cipher);

    let mut best_fitness = scoring::get_text_score(&deciphered, 4, &map);
    let mut local_best_fitness = best_fitness;

    let mut best_key = playfair.get_key();
    let mut local_best_key = best_key;

    let mut temp = 10.0 + 0.087 * (cipher.len() - 84) as f64;
    let mut rng = thread_rng();

    while temp >= 0.0 {
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
                println!("Current Temp: {:?}", temp);
                println!("Best fitness: {:?}", best_fitness);
                println!("Best key: {:?}", best_key);
                println!("Deciphered: {:?}", deciphered);
            }
        }

        temp -= DELTA_TEMP;
        println!("Cooling: {:?}. Less fit children chosen: {}", temp, less_fit_count);
    }

}
