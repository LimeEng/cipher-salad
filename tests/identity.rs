use cipher_salad::{
    Alphabet, ForeignGraphemesPolicy,
    cipher::{Atbash, Caesar, Vigenere},
};
use quickcheck_macros::quickcheck;

#[quickcheck]
#[allow(clippy::needless_pass_by_value)]
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
#[allow(clippy::needless_pass_by_value)]
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
#[allow(clippy::needless_pass_by_value)]
fn atbash_identity(plaintext: String) -> bool {
    let alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let alphabet = Alphabet::new(alphabet);
    let cipher = Atbash::new(alphabet, ForeignGraphemesPolicy::Include);

    let ciphertext = cipher.encrypt(&plaintext);
    let decrypted = cipher.decrypt(&ciphertext);

    plaintext == decrypted
}
