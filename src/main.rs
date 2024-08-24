// mod arp;
pub mod arp;
pub mod find_ips;
pub mod password_craker;

use password_craker::create_users_from_shadow;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use sha_crypt::{CryptError, Sha512Params};
use std::fs; // Constatns

fn main() {

    let users = create_users_from_shadow();

    for user in &users {
        let password = user.crack_password("10k-most-common.txt");
    }

}
