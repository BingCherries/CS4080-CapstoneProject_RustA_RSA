# CS4080: Concepts of Programming Languages (Capstone Project)
# Team: Rust A
Members: Cody Apolinar, Kelvin Quizon, Thomas Christopher Tejedor, Yunseon Choi

# RSA Encryption and Decryption in Rust


## Overview
This project implements a simplified version of RSA encryption and decryption in Rust. It demonstrates key cryptographic principles, such as key pair generation, modular arithmetic, and secure message transmission. The program encrypts and decrypts a message to illustrate the end-to-end process of secure communication.

## Problem Statement
Secure communication is a critical challenge, especially when transmitting sensitive data over insecure channels. The goal of this project is to demonstrate how RSA encryption can ensure message confidentiality by encoding messages in a way that only authorized parties can decode.

## How to Run
1. Build the program:
   ```
   cargo build
   ```
2. Run the program:
  ```
  cargo run
  ```
3. Example Output:
   ```
   Public Key, Private Key, Modulo Val
   (5, 173, 481)

   Message to be Encrypted
   "Test Message"

   Encrypted Message
   [470, 381, 358, 350, 428, 381, 358, 358, 119, 51, 381]

   Decrypted Message
   "Test Message"
   ```

## Reference
https://github.com/RustCrypto/RSA 

https://www.geeksforgeeks.org/rsa-algorithm-cryptography/
