pub trait EnclaveCrypto {
    type Error;
    type CryptoPair;
    type Seed;

    fn create_random_pair() -> Result<(Self::CryptoPair, Self::Seed), Self::Error>;
    fn get_pair_from_seed(seed: Self::Seed) -> Result<Self::CryptoPair, Self::Error>;
}