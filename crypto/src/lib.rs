use rand_core::{OsRng, RngCore};

pub mod traits;
pub mod ed25519;
pub mod ecdsa;
pub mod sr25519;


fn create_random_seed() -> [u8; 32] {
    let mut seed = [0u8; 32];
    OsRng.fill_bytes(&mut seed);
    seed
}

#[cfg(test)]
mod tests {
    use crate::ed25519::Ed25519;
    use crate::traits::EnclaveCrypto;
    use sp_core::Pair;
    use crate::ecdsa::Ecdsa;
    use crate::sr25519::Sr25519;


    macro_rules! test_pair_creation {
        ($func_name:ident, $crypto:ident) => {
            #[test]
            fn $func_name() {
                let (pair, seed) = $crypto::create_random_pair().expect(&("Unable to create random ".to_owned()+stringify!($crypto)+" pair"));
                let recreate_pair = $crypto::get_pair_from_seed(seed).expect(&("Unable to generate ".to_owned()+stringify!($crypto)+" pair from seed"));
                assert_eq!(pair.public(),recreate_pair.public());
                let (new_pair, new_seed) = $crypto::create_random_pair().expect(&("Unable to create random ".to_owned()+stringify!($crypto)+" pair"));
                assert!(seed != new_seed);
                assert!(new_pair.public() != pair.public());
            }
        };
}

    test_pair_creation!(test_ed25519_pair,Ed25519);
    test_pair_creation!(test_ecdsa_pair,Ecdsa);
    test_pair_creation!(test_sr25519_pair,Sr25519);
}
