use crate::traits::EnclaveCrypto;
use sp_core::{ed25519, Pair};
use sp_core::crypto::SecretStringError;
use crate::create_random_seed;

/// API to create Ed25519 pair inside enclave
pub struct Ed25519;

impl EnclaveCrypto for Ed25519 {
    type Error = SecretStringError;
    type CryptoPair = ed25519::Pair;
    type Seed = [u8; 32];

    fn create_random_pair() -> Result<(Self::CryptoPair, Self::Seed), Self::Error> {
        let seed = create_random_seed();
        let pair = ed25519::Pair::from_seed_slice(&seed)?;
        Ok((pair, seed))
    }

    fn get_pair_from_seed(seed: Self::Seed) -> Result<Self::CryptoPair, Self::Error> {
        ed25519::Pair::from_seed_slice(&seed)
    }
}

