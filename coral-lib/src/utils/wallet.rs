use ethers::prelude::{rand, LocalWallet};

/// Generate a random wallet.
/// Mostly used for calling view and pure functions
pub fn generate_random_wallet() -> LocalWallet {
    LocalWallet::new(&mut rand::thread_rng())
}
