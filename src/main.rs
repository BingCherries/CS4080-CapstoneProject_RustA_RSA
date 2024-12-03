//module to use rand numbers
use rand::Rng;
use gcd; 

fn main() {
    //returns keys (0: public 1: private 2:n i.e. modular val)
    let keys = set_keys();
    let message: String = String::from("Test Message");

    println!("Public Key, Private Key, Modulo Val");
    println!("{:?}\n", keys);

    println!("Message to be Encrypted");
    println!("{:?}\n", message);


    let encoded_message = encoder(message, keys);
    
    println!("Encrypted Message");
    println!("{:?}\n", encoded_message);

    let decrypted_message = decoder(encoded_message, keys);

    println!("Decrypted Message");
    println!("{:?}", decrypted_message);
}

fn encrypt(letter: u64 , keys: (u64,u64,u64)) -> u64 {
    let mut e: u64 = keys.0;
    let mut encrypted_text: u64 = 1;
    //(letter ^ public key) mod n
    while e != 0 {
        encrypted_text *= letter; 
        encrypted_text %= keys.2;
        e -= 1; 
    }
    return encrypted_text;

}

fn decrypt(encrypted_letter: u64, keys: (u64,u64,u64)) -> u32{
    let mut d: u64 = keys.1;
    let mut decrypted: u64 = 1;
    //(encrypted_letter ^ private key) mod n
    while d != 0 {
        decrypted *= encrypted_letter;
        decrypted %= keys.2;
        d -= 1;
    }
    return decrypted.try_into().unwrap();
}

fn encoder(message: String, keys: (u64, u64, u64)) -> Vec<u64>{

    
    let mut form = Vec::new();

    //Encrypt every letter of the string
    for c in message.chars() {
        
        form.push(encrypt(c as u64, keys));
    }
    //return a vector of every encrypted letter in u64 form 
    return form;
}

fn decoder(encoded: Vec<u64>, keys: (u64,u64,u64)) -> String{
    let mut s: String = String::from("");

    //Decrypt every character 
    for n in encoded {
        s.push(char::from_u32(decrypt(n,keys)).unwrap());
    }
    //return decoded string
    return s; 
}

//Get the keys to encrypt and decrypt (public, private, n i.e. value to mod); 
fn set_keys() -> (u64, u64, u64){
    let prime1 = return_rand_prime();
    let prime2 = return_rand_prime();

    //first part of private key
    let n = prime1 * prime2;
    //totient value
    let fi: u64 = ((prime1 - 1) * (prime2 - 1)).into();
    //primary key is relatively prime to totient value
    let mut e: u64 = 2; 
    loop {
        //greatest common divisor
        if gcd::binary_u64(e,fi) == 1 {
            break;
        }
        e += 1;
    }
    let public_key: u64 = e;
    let mut d: u64 = 2;
    //find private key i.e private key * public key / totient value must have value of 1
    loop {
        if (d*e) % fi == 1 {
            break;
        }
        d += 1;
    }
    let private_key: u64  = d; 
    return (public_key, private_key, n.into());
} 



//Return a random prime number
fn return_rand_prime() -> u32 {

    //List of all primes from 0-300
    let mut primes = vec![11, 13, 17, 19, 23, 29, 31, 37, 
    41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97];

    //get random prime from the vector of prime numbers
    let rng = rand::thread_rng().gen_range(0..primes.len());
    let prime: u32 = primes[rng];
    //remove prime from vector so it is not repeated
    primes.remove(rng);
    return prime; 
}