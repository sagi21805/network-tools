use base64::{engine::general_purpose, Engine as _};
use sha_crypt::{sha512_crypt_b64, CryptError, Sha512Params};
use std::fs;
use std::os::unix::process::ExitStatusExt;
use std::process::{Command, ExitStatus, Output};
use std::str;
use std::thread::ScopedJoinHandle;
use std::{fmt, string::FromUtf8Error}; // Constatns
use core::str::Split;
use rayon::prelude::*;

// Constants
const HASH_IDX: usize            = 1;
const NAME_IDX: usize            = 0;
const TYPE_IDX: usize            = 0;
const SALT_IDX: usize            = 1;
const PWD_HASH_IDX: usize        = 2;
const ITERATIONS: usize          = 5000;
const SHADOW_PATH: &str          = "/etc/shadow";


// User class to easly handle shadow
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct User {
    pub name: String,
    pub pwd_hash: String,
    pub salt: String,
    pub hash_type: String
}


impl User {

    /// A function for creating a new user
    pub fn new(name: String, pwd_hash: String, salt: String, hash_type: String) -> Self {
        User {
            name,
            pwd_hash,
            salt,
            hash_type,
        }
    }


    // compare the guess with the hash of the real password
    pub fn guess_password(&self, guess: &str, params: &Sha512Params) -> Result<String, CryptError> {
        let salt_bytes = self.salt.as_bytes().to_owned();
        let password_bytes = guess.as_bytes().to_owned();
        sha512_crypt_b64(&password_bytes, &salt_bytes, params)
    }


    pub fn crack_password(&self, passwords_file: &str) -> Option<String>{

        let hash_params: Sha512Params = Sha512Params::new(ITERATIONS).unwrap();
        if self.hash_type != "6" { return None; }

        let weak_passwords_data = fs::read_to_string(
            passwords_file
        ).expect("Should have been able to read the file");

        let weak_passwords = weak_passwords_data.split("\n").collect::<Vec<&str>>();

        let password = weak_passwords.clone().into_par_iter().find_any(|&guess| {
            let guess_hash = match self.guess_password(guess, &hash_params) {
                Err(_) => panic!("Coudln't guess password"),
                Ok(hash) => hash
            };
            return guess_hash == self.pwd_hash 
        });
        println!("Password for user {} is '{}'", self.name, password.unwrap());

        return Some(password.unwrap().to_string());
    }
}


impl fmt::Display for User {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Username: {}, pwd_hash: {}, salt: {}",
            self.name, self.pwd_hash, self.salt
        )
    }
}


pub fn create_users_from_shadow() -> Vec<User> {

    let mut users: Vec<User> = Vec::<User>::new();
    let shadow_content = fs::read_to_string(SHADOW_PATH).expect("Can't read file");
    let shadow_lines = shadow_content.split("\n");
    
    // Generate users from lines in shadow
    for line in shadow_lines {
        let details = line.split(":").collect::<Vec<&str>>();

        if details.len() < 2 { continue; } // min len required for username and hash
        if !details[HASH_IDX].contains("$") { continue; } // Check not service user

        // Get that hash deatils and skip the first split because it's empty
        let hash_details = details[HASH_IDX].split("$").skip(1).collect::<Vec<&str>>();

        // Get the name, pwd_hash, salt and hash_type from the details to create the user
        let name = details[NAME_IDX].to_string();
        let pwd_hash = hash_details[PWD_HASH_IDX].to_string();
        let salt = hash_details[SALT_IDX].to_string();
        let hash_type = hash_details[TYPE_IDX].to_string();

        users.push(User::new(name, pwd_hash, salt, hash_type));
    }

    users
}

fn guess_wifi_password(net_name: &str, guess: &str) -> ExitStatus{
    Command::new("sh")
    .arg("-C")
    .arg("src/password_craking/scripts/connect_wifi.sh")
    .arg(net_name)
    .arg(guess)
    .status()
    .expect("failed to execute process")
}

pub fn crack_wifi_password(net_name: &str, passwords_file: &str) {
    let weak_passwords_data = fs::read_to_string(
        passwords_file
    ).expect("Should have been able to read the file");

    let weak_passwords = weak_passwords_data.split("\n").collect::<Vec<&str>>();


    let password = weak_passwords.clone().into_par_iter().find_first(|&guess| {
        return guess_wifi_password(net_name, guess).success();
    });
    println!("Password for net {} is '{}'", net_name, password.expect("expected password"));

}