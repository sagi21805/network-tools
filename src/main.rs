// mod arp;
mod networking;
mod password_craking;
mod test;

use std::{process::Command, u16::MAX};
use password_craking::*;
use networking::*;

use rayon::prelude::*;
use password_craker::create_users_from_shadow;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use sha_crypt::{CryptError, Sha512Params};
use std::fs; // Constatns
use port_scanner::scan_port_addr;

fn scan_all_ports(ip: &str) -> Vec<u16>{
    let mut open = Vec::<u16>::new();
    (0..=u16::MAX).into_iter().for_each(|port|{
        match scan_port_addr(format!("{}:{}", ip, port)){
            true => open.push(port),
            false => ()
        }
    });
    return open;
}

fn main() {
    // let out = Command::new("sh")
    //     .arg("-c")
    //     .arg(r"sudo arp-scan -lq | grep -Eo '([0-9]*\.){3}[0-9]{1,3}'")
    //     .output()
    //     .expect("Failed to execute command");

    // let output = String::from_utf8_lossy(&out.stdout);
    // let lines = output.split("\n").collect::<Vec<&str>>();
    // println!("ip's: {:?}", lines);
    // lines.into_par_iter().for_each(|ip| {
    //     let out = Command::new("sh")
    //     .arg("-c")
    //     .arg(format!(r"nmap -p- {}", ip))
    //     .output()
    //     .expect("Failed to execute command");

    //     let output = String::from_utf8_lossy(&out.stdout);
    //     println!("{}", output);
    // })


    password_craker::crack_wifi_password("Michal", "10k-most-common.txt");

}
