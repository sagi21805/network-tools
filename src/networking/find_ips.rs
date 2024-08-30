use pinger::ping;
use pnet::datalink;
use rayon::prelude::*;

const MAX_MASK_PREFIX: u8 = 32;

pub fn find_ips() {
    let interfaces: Vec<datalink::NetworkInterface> = datalink::interfaces();
    for interface in interfaces {
        println!("interface: {}", interface);
        for ip_network in interface.ips {
            let network = ip_network.network();
            let net_string = network.to_string();
            let mut tested_address: Vec<String> =
                net_string.split(".").map(|str| str.to_string()).collect();

            if (!network.is_ipv4() || tested_address[0] == "127") {
                continue;
            }
            println!("My ip: {}", ip_network.ip());

            let mask_prefix = ip_network.prefix();
            // let num_of_loops = (MAX_MASK_PREFIX - mask_prefix) / 8;
            (0u8..255).into_par_iter().for_each(|i| {
                let mut tested_address: Vec<String> =
                    net_string.split(".").map(|str| str.to_string()).collect();
                tested_address[3] = i.to_string();
                let addr = tested_address.join(".");
                match ping(addr, None).expect("Error pinging").recv().unwrap() {
                    pinger::PingResult::Pong(_, address) => {
                        println!("success pinging: {}", address);
                    }

                    _ => {}
                }
            })
        }
    }

    // stream.
}
