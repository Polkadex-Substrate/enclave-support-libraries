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

/// This trait should be implemented by all crypto schemes to be accessible inside enclave
pub trait EnclaveCrypto {
    type Error;
    type CryptoPair;
    type Seed;

    /// Creates a random seed, pair of the cryptographic scheme
    fn create_random_pair() -> Result<(Self::CryptoPair, Self::Seed), Self::Error>;
    /// Creates the pair using given seed
    fn get_pair_from_seed(seed: Self::Seed) -> Result<Self::CryptoPair, Self::Error>;
}