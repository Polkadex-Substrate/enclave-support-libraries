use crate::traits::EnclaveCrypto;
use sp_core::{ecdsa, Pair};
use sp_core::crypto::SecretStringError;
use crate::create_random_seed;

/// API to create ECDSA pair inside enclave
pub struct Ecdsa;

impl EnclaveCrypto for Ecdsa {
    type Error = SecretStringError;
    type CryptoPair = ecdsa::Pair;
    type Seed = [u8; 32];

    fn create_random_pair() -> Result<(Self::CryptoPair, Self::Seed), Self::Error> {
        let seed = create_random_seed();
        let pair = ecdsa::Pair::from_seed_slice(&seed)?;
        Ok((pair, seed))
    }

    fn get_pair_from_seed(seed: Self::Seed) -> Result<Self::CryptoPair, Self::Error> {
        ecdsa::Pair::from_seed_slice(&seed)
    }
}
