use std::net::Ipv4Addr;

// mod arp;
mod arp;
mod find_ips;

use pnet::datalink::MacAddr;

fn main() {
    // find_ips::find_ips();
    // ethernet::send_arp_req(Ipv4Addr::new(10, 100, 102, 7))
    loop {
        arp::send_arp_res(
            MacAddr::new(0xb4, 0x2e, 0x99, 0x5a, 0x99, 0xb5), 
            Ipv4Addr::new(10, 100, 102, 7)
        );
        println!(
            "Running"
        );
        std::thread::sleep(std::time::Duration::from_millis(500))
    }
}
