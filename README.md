# Rust RSA Visualization

The goal of this project is to re-implement the RSA hashing algorithm in Rust, separately from the existing crate. We also plan to implement secure (ish) pseudo-random number generation, for use with the hashing algorithm. If time so permits, we would also like to create some visualization of the steps the algorithm takes when operating on the data being hashed.

## TODO:
- [ ] Implement pseudo-random number generation using linear congruential method (recursively)
    - [ ] Check randomness using frequency test
    - [ ] Check randomness using chi squared test?
    - [ ] Bitmap visualization of random numbers? (as seen [here](https://www.random.org/analysis/))
- [ ] Implement prime number generation
    - [ ] Using Sieve of Eratosthenes
    - [ ] Using Sieve of Atkin ([wikipedia](https://en.wikipedia.org/wiki/Sieve_of_Atkin), [geeksforgeeks](https://www.geeksforgeeks.org/sieve-of-atkin/))
    - [ ] Look at number field sieves?
- [ ] Implement RSA Encryption algorithm
    - [ ] Using our RNG
    - [ ] Using system RNG (e.g. /dev/urandom)
    - [ ] Using Rust `rand` crate
- [ ] Test RSA Implementation
    - [ ] Compare performance (as in speed) with Rust `rsa` crate
    - [ ] Test security of implementation using [NIST RSA2VS](https://csrc.nist.gov/CSRC/media/Projects/Cryptographic-Algorithm-Validation-Program/documents/dss2/rsa2vs.pdf)? (from FIPS 186-4)
- [ ] Make GUI?
