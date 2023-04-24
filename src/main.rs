use std::{str, env};
use rand::rngs::OsRng;
use rsa::{PublicKey, RSAPrivateKey, PaddingScheme};
use num_traits::One;

fn main() {

    let mut rng = OsRng;   // zero struct to gen a random number
    let mut bits = 16; 
    let mut string = String::from("Hello world!");

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 { 
        string = args[1].clone();   // fix using ref instead of cloning
    }
    if args.len() > 2 { 
        bits = args[2].clone().parse::<usize>().unwrap(); // parse &str into usize
    } 

    // rng is the range of the exponent
    let key = RSAPrivateKey::new(&mut rng, bits).expect("failed to generate a key");

    println!("Message:\t{}",string);
    println!("Number of bits:\t{}",bits);
    
    let data = string.as_bytes();  // returns an array with refereces of type u8

    // e, d, N, primes
    println!("\nN:\t{} (Hex: {:x})",key.n(),key.n());
    println!("E:\t{} (Hex: {:x})",key.e(),key.e());
    println!("D:\t{} (Hex: {:x})",key.d(),key.d() );
    println!("\nPrimes (P and Q):");

    for prime in key.primes() {
        println!("\t{} (Hex:{:x})",prime,prime);
    }

    let enc_data = key.encrypt(&mut rng, PaddingScheme::PKCS1v15, &data[..]).expect("failed to encrypt");

    let hex_string = hex::encode(enc_data.clone());  // convert encoded data to Hex
    println!("\n\nEncrypted:\t{}",hex_string);

    // decode and convert back to string
    let dec_data = key.decrypt(PaddingScheme::PKCS1v15, &enc_data).expect("failed to decrypt");
    let mystr = str::from_utf8(&dec_data).unwrap();
    println!("\nDecrypted :\t{}",mystr);

    // prime check
    let p=  key.primes()[0].clone();
    let q=  key.primes()[1].clone();
    let val1: rsa::BigUint = One::one();   // multip identidy
    let phi = (p - val1.clone()) * (q - val1.clone());
    let val = (key.d()*key.e()) % phi;
    println!("\n(d*e) mod (p-1)(q-1):\t{}",val);

}