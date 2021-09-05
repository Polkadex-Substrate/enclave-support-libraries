// This file is part of Polkadex.

// Copyright (C) 2020-2021 Polkadex OU.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use rand_core::{OsRng, RngCore};

pub mod traits;
pub mod ed25519;
pub mod ecdsa;
pub mod sr25519;

/// Generates a random seed of 32 bytes in size
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
