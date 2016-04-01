# Playfair Hammer #

A program that cracks the [Playfair cipher](https://en.wikipedia.org/wiki/Playfair_cipher) using [simulated annealing](https://en.wikipedia.org/wiki/Simulated_annealing) in Rust.

## Usage ##
The program takes cipher text input from STDIN so the easiest way is to use
cargo to run the program:

```$ cargo run --release < example_cipher.txt```

where ```example_cipher.txt``` is a text file containing the cipher to be cracked
(an example file has been provided).

It may be desirable to play around with the constants ```DELTA_TEMP``` and
```CYCLE_SIZE``` located in ```src/main.rs```. These constants indicate by how
much the program should reduce the temperature after each cycle and how large
each cycle is respectively.

## Notes ##
* The algorithm only takes a couple of seconds to finish when dealing with
ciphers less than 100 characters long, but may not arrive at an accurate
decryption.
* Longer ciphers take longer to process but are more likely to churn out an
accurate decryption.
* The program uses the following equation to determine an appropriate starting temperature:
```10 + 0.087 * cipher_text_length - 84``` as described in Cowan's [article](http://www.tandfonline.com/doi/abs/10.1080/01611190701743658).
* The quadgram file used for scoring trial decryptions was taken from [practical cryptography](http://practicalcryptography.com/media/cryptanalysis/files/english_quadgrams.txt.zip)
