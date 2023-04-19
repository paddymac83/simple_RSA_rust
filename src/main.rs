use std::convert::TryFrom;

fn main() {

    // create inital RSA struct with chosen primes
    let RSA_enc = RsaEncoder::new();
    let mod_N = RSA_enc.calculate_mod_N();
    let exp = RSA_enc.calculate_exponent();
    let dec = RSA_enc.calculate_decrypt();

}

struct RsaEncoder {
    lower_prime: usize,
    upper_prime: usize,
    sieves,
}

struct Message {
    original: String,
    encrypted: [&u8],
    decrypted: String,
}

impl RsaEncoder {
    fn new() -> Self {
    // calculate the upper limit for the nth prime
    // cache all primes up to that limit
    let  (_lo, hi) = slow_primes::estimate_nth_prime(100);
    let sieves = slow_primes::Primes::sieve(hi as usize);

    // match two primes below limit for encryption
    let lower_prime = match sieves.primes().nth(50) {
        Some(lower_prime) => lower_prime,
        None => unreachable!(),
    };

    let upper_prime = match sieves.primes().nth(60) {
        Some(upper_prime) => upper_prime,
        None => unreachable!(),
    };

    Self {
        lower_prime,
        upper_prime,
        sieves,
    }
}

    fn calculate_mod_N(&self) -> usize {
        self.lower_prime*self.upper_prime
    }

    fn calculate_exponent(&self) -> usize {
        // use pre-computed primes to find e such that it doesnt divide into p-1 and q-1
        let prime1 = self.upper_prime-1;
        let prime2 = self.lower_prime-1;

        // iterate over list of Primes to find e
        for i in self.sieves {
            if prime1 % i == 0 & prime2 % i == 0 {
                return i
            } 
        }
    }

    fn calculate_decrypt(&self) -> usize {

    }
    
}
