use cipher_salad::Alphabet;
use cipher_salad::ForeignGraphemesPolicy;
use cipher_salad::cipher::{Atbash, Caesar, Vigenere};
use quickcheck_macros::quickcheck;

#[quickcheck]
fn vigenere_identity(plaintext: String) -> bool {
    let alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let alphabet = Alphabet::new(alphabet);
    let cipher = Vigenere::new(alphabet, ForeignGraphemesPolicy::Include);

    let key = "secret";
    let ciphertext = cipher.encrypt(&plaintext, key).unwrap();
    let decrypted = cipher.decrypt(&ciphertext, key).unwrap();

    plaintext == decrypted
}

#[quickcheck]
fn caesar_identity(plaintext: String) -> bool {
    let alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let alphabet = Alphabet::new(alphabet);
    let cipher = Caesar::new(alphabet, ForeignGraphemesPolicy::Include);

    let key = 9;
    let ciphertext = cipher.encrypt(&plaintext, key);
    let decrypted = cipher.decrypt(&ciphertext, key);

    plaintext == decrypted
}

#[quickcheck]
fn atbash_identity(plaintext: String) -> bool {
    let alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let alphabet = Alphabet::new(alphabet);
    let cipher = Atbash::new(alphabet, ForeignGraphemesPolicy::Include);

    let ciphertext = cipher.encrypt(&plaintext);
    let decrypted = cipher.decrypt(&ciphertext);

    plaintext == decrypted
}
