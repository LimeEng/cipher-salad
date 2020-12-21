use cipher_salad::Alphabet;
use cipher_salad::ForeignGraphemesPolicy;
use cipher_salad::Vigenere;

use quickcheck_macros::quickcheck;

#[quickcheck]
fn encrypt_decrypt_identity(plaintext: String) -> bool {
    let alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let alphabet = Alphabet::new(alphabet.to_string());
    let cipher = Vigenere::new(alphabet, ForeignGraphemesPolicy::Include);

    let key = "secret";
    let ciphertext = cipher.encrypt(&plaintext, key).unwrap();
    let decrypted = cipher.decrypt(&ciphertext, key).unwrap();

    plaintext == decrypted
}
