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
