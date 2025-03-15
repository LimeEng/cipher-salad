use cipher_salad::{
    Alphabet, ForeignGraphemesPolicy,
    cipher::{Atbash, Caesar, Vigenere},
};
fn main() {
    vigenere();
    caesar();
    atbash();
}

fn vigenere() {
    let policy = ForeignGraphemesPolicy::Include;
    let alphabet = "abcdefghijklmnopqrstuvwxyz";
    let alphabet = Alphabet::new(alphabet);
    let cipher = Vigenere::new(alphabet, policy);
    let plaintext = "Hello world!";
    let key = "secret";
    let ciphertext = cipher.encrypt(plaintext, key).unwrap();
    let decrypted = cipher.decrypt(&ciphertext, key).unwrap();
    println!("Vigenere: ");
    println!("    {plaintext}");
    println!("    {ciphertext}");
    println!("    {decrypted}");
    println!("{}", "=".repeat(plaintext.len()));
}

fn caesar() {
    let policy = ForeignGraphemesPolicy::Include;
    let alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ 1234567890!.,";
    let alphabet = Alphabet::new(alphabet);
    let cipher = Caesar::new(alphabet, policy);
    let plaintext = "Hello world!";
    let key = 9;
    let ciphertext = cipher.encrypt(plaintext, key);
    let decrypted = cipher.decrypt(&ciphertext, key);
    println!("Caesar: ");
    println!("    {plaintext}");
    println!("    {ciphertext}");
    println!("    {decrypted}");
    println!("{}", "=".repeat(plaintext.len()));
}

fn atbash() {
    let policy = ForeignGraphemesPolicy::Include;
    let alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ 1234567890!.,";
    let alphabet = Alphabet::new(alphabet);
    let cipher = Atbash::new(alphabet, policy);
    let plaintext = "Hello world!";
    let ciphertext = cipher.encrypt(plaintext);
    let decrypted = cipher.decrypt(&ciphertext);
    println!("Atbash: ");
    println!("    {plaintext}");
    println!("    {ciphertext}");
    println!("    {decrypted}");
    println!("{}", "=".repeat(plaintext.len()));
}
