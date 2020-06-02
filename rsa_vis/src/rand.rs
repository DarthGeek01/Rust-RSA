use std::time::{SystemTime, UNIX_EPOCH};
use std::alloc::System;
use std::intrinsics::likely;

/// The modulus constant from the GCC implementation of rand (2^31). We can make this more efficient
/// with binary trickery if we so choose, since it's just a power of two.
const GCC_MOD: u64 = 2147483648;
/// The multiplication constant from GCC
const GCC_MULT: u64 = 1103515245;
/// The increment constant from GCC
const GCC_INC: u64 = 12345;
/// The number of iterations I have decided is appropriate
const ITERATIONS: u64 = 200;

/// This struct represents a random number generator using the linear congruential method (L.C.M.),
/// since RNG is ultimately a sequence. `Rng` is not accessible outside of the `rand` module,
/// creation will have two layers of abstraction.
struct Rng {
    /// Multiplier
    a: u64,
    /// Current value
    x: u64,
    /// Increment
    c: u64,
    /// Modulus
    m: u64,
}

impl Rng {
    /// Returns a new random number generator with the parameters specified here
    ///
    /// # Arguments
    ///     * `modulus` - The modulus value for L.C.M.
    ///     * `multiplier` - The multiplier for L.C.M.
    ///     * `increment` - The increment for L.C.M.
    ///     * `seed` - Initial value for L.C.M. sequence
    fn new(modulus: u64, multiplier: u64, increment: u64, seed: u64) -> Rng {
        let mut ret = Rng {
            a: multiplier,
            x: seed,
            c: increment,
            m: modulus
        };
        ret.next(); // So we don't just return the seed as the first value

        ret
    }

    /// Return the next random number in the sequence, normalized as a value in the range [0..1]
    fn next(&mut self) -> f64 {
        self.x = (a * self.x + self.c) % self.m;
        (self.x as f64) / (self.m as f64)
    }

    // Return the next random number in the sequence, as an integer between `min` and `max`
    fn next_int(&mut self, min: u64, max: u64) -> u64 {
        let range = max - min;

        min + ((self.next() * (min as f64)) as u64) as u64
    }
}

/// Returns a new RNG object, seeded with the explicitly provided seed
pub fn new_seed(seed: u64) -> Rng {
    Rng::new(GCC_MOD, GCC_MULT, GCC_INC, seed)
}

/// Returns a new RNG object, seeded with the current Unix time in seconds
pub fn new() -> Rng {
    let seed: u64 = SystemTime::now().duration_since(UNIX_EPOCH)
        .expect("oh balls")
        .as_secs();

    new_seed(seed)
}
