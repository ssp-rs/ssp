use rand_chacha::{
    rand_core::{RngCore, SeedableRng},
    ChaCha20Rng,
};

use num_bigint::BigUint;
use num_traits::cast::ToPrimitive;

use crate::Seed;

mod small_primes;

use small_primes::*;

// NIST recomends 5 rounds for miller rabin. This implementation does 8. Apple uses 16. Three iterations has a probability of 2^80 of failing
const MILLER_RABIN_ROUNDS: usize = 16;
const ZERO: u64 = 0;
const ONE: u64 = 1;
const TWO: u64 = 2;

pub struct Generator {
    seed: Seed,
    rng: ChaCha20Rng,
}

impl Generator {
    /// Creates a new [Generator] from the provided [Seed].
    ///
    /// The [Seed] is used to generate cryptographically secure random numbers, and should come
    /// from a source of high-grade entropy.
    ///
    /// See the [Entropy (computing) - Wikipedia](https://en.wikipedia.org/wiki/Entropy_(computing)#Embedded_systems) article for general pitfalls to avoid.
    ///
    /// As a last resort, users may want to generate seeds on systems with good sources of entropy,
    /// and transfer the seed to the embedded system. If that technique is used, unique seeds
    /// should be generated per-device.
    pub fn from_seed(seed: Seed) -> Self {
        Self {
            seed,
            rng: ChaCha20Rng::from_seed(seed),
        }
    }

    /// Creates a new [Generator] using system entropy.
    ///
    /// Generates the seed from system entropy, see [getrandom](https://docs.rs/getrandom/latest).
    #[cfg(feature = "std")]
    pub fn from_entropy() -> Self {
        let mut seed = [0u8; 32];

        rand::thread_rng().fill_bytes(&mut seed);

        Self {
            seed,
            rng: ChaCha20Rng::from_seed(seed),
        }
    }

    /// Generates a new prime.
    pub fn new_prime(&mut self) -> u64 {
        loop {
            let candidate = self.rng.next_u64();

            if Self::is_prime(candidate, &mut self.rng) {
                return candidate;
            }
        }
    }

    /// Sets the CSPRNG seed to a new value.
    pub fn set_seed(&mut self, seed: Seed) {
        self.seed = seed;
        self.rng = ChaCha20Rng::from_seed(self.seed);
    }

    fn is_prime(candidate: u64, rng: &mut ChaCha20Rng) -> bool {
        candidate != ZERO
            && (Self::is_odd(candidate) || candidate == TWO)
            && Self::div_small_primes(candidate)
            && Self::fermat(candidate, rng)
            && Self::miller_rabin(candidate, rng)
    }

    fn is_even(n: u64) -> bool {
        n % TWO == 0
    }

    fn is_odd(n: u64) -> bool {
        !Self::is_even(n)
    }

    fn gen_range(low: u64, high: u64, rng: &mut ChaCha20Rng) -> u64 {
        use rand::distributions::uniform::{UniformInt, UniformSampler};

        let uniform = UniformInt::<u64>::new_inclusive(low, high);
        uniform.sample(rng)
    }

    fn div_small_primes(candidate: u64) -> bool {
        for p in SMALL_PRIMES.iter().map(|&p| p as u64) {
            if candidate == p {
                return true;
            }
            if candidate % p == 0 {
                return false;
            }
        }

        true
    }

    fn fermat(candidate: u64, rng: &mut ChaCha20Rng) -> bool {
        for _ in 0..MILLER_RABIN_ROUNDS {
            let a = Self::gen_range(TWO, candidate - TWO, rng);

            let ab = BigUint::from(a);
            let exp = BigUint::from(candidate - ONE);
            let mb = BigUint::from(candidate);

            if ab.modpow(&exp, &mb).to_u64().unwrap() != ONE {
                return false;
            }
        }

        true
    }

    fn miller_rabin(candidate: u64, rng: &mut ChaCha20Rng) -> bool {
        if candidate == TWO {
            return true;
        }

        let (d, s) = Self::rewrite(candidate);

        let step = s.saturating_sub(ONE);
        let two = BigUint::from(TWO);

        let db = BigUint::from(d);
        let nb = BigUint::from(candidate);

        for _ in 0..MILLER_RABIN_ROUNDS {
            let a = Self::gen_range(TWO, candidate - ONE, rng);

            // (a ^ d mod n)
            let ab = BigUint::from(a);

            let mut x = ab.modpow(&db, &nb).to_u64().unwrap_or(ONE);

            if x == ONE || x == (candidate - ONE) {
                continue;
            } else {
                let mut break_early = false;

                for _ in ZERO..step {
                    let xb = BigUint::from(x);
                    x = xb.modpow(&two, &nb).to_u64().unwrap_or(ONE);

                    if x == ONE {
                        return false;
                    } else if x == (candidate - ONE) {
                        break_early = true;
                        break;
                    }
                }

                if !break_early {
                    return false;
                }
            }
        }

        true
    }

    fn rewrite(n: u64) -> (u64, u64) {
        let mut s = ZERO;

        let mut d = n - ONE;

        while Self::is_even(d) {
            d /= TWO;
            s += ONE;
        }

        (d, s)
    }
}