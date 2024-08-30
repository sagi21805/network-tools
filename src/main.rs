// mod arp;
mod networking;
mod password_craking;

use password_craking::*;
use networking::*;

use password_craker::create_users_from_shadow;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use sha_crypt::{CryptError, Sha512Params};
use std::fs; // Constatns

fn main() {

    let users = create_users_from_shadow();

    println!("users: {:?}", users);

    for user in &users {
        let password = user.crack_password("10k-most-common.txt");
    }

}
