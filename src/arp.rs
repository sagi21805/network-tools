extern crate pnet;

use std::net::Ipv4Addr;
use std::sync::mpsc::Receiver;
use std::thread;
use std::time::Duration;

use mac_address::{get_mac_address, MacAddress};
use extend::ext;

use pnet::datalink::Channel;
use pnet::datalink::{self, NetworkInterface};
use pnet::packet::arp::MutableArpPacket;
use pnet::packet::arp::{ArpHardwareTypes, ArpOperation, ArpOperations};
use pnet::packet::ethernet::EtherTypes;
use pnet::packet::ethernet::MutableEthernetPacket;
use pnet::packet::MutablePacket;
use pnet::util::MacAddr;


#[ext]
impl MacAddress {
    fn into_MacAddr(self) -> MacAddr {
        let [b0, b1, b2, b3, b4, b5] = self.bytes();
        pnet::util::MacAddr::new(b0, b1, b2, b3, b4, b5)
    }
}

fn get_local_ip() -> Option<Ipv4Addr> {
    // Iterate over the available network interfaces.
    let interfaces = datalink::interfaces();
    for interface in interfaces {
        for ip_network in interface.ips {
            // Check if the IP address is an IPv4 address.
            if let std::net::IpAddr::V4(ipv4) = ip_network.ip() {
                // Return the first non-loopback IPv4 address found.
                if !ipv4.is_loopback() {
                    return Some(ipv4);
                }
            }
        }
    }
    // Return None if no valid IP address is found.
    None
}

fn get_local_interface() -> Option<NetworkInterface> {
    
    let interfaces = datalink::interfaces();
    for interface in interfaces {
        println!("interface: {}", interface);
        if !interface.is_loopback(){
            return Some(interface);
        }
    }
    return None;
}

pub fn send_arp_req(
    target_ip: Ipv4Addr,
) {

    let local_interface = {
        match get_local_interface() {
            Some(interface) => { interface }
            None => panic!("Can't find interface")
        }

    };

    let (mut sender, receiver) = match datalink::channel(
        &local_interface, Default::default()) {

        Ok(Channel::Ethernet(
            sender, receiver
        )) => (sender, receiver),

        Ok(_) => panic!("Unknown channel type"),
        Err(e) => panic!("Error happened {}", e),
    };

    let mut eth_buf = [0u8; 42];

    let mut eth_packet = MutableEthernetPacket::new(
        &mut eth_buf
    ).unwrap();

    eth_packet.set_destination(MacAddr::broadcast());
    
    let source_mac = {
        match get_mac_address() {
            Ok(Some(ma)) => { ma }
            Ok(None) => { panic!("Mac address wasn't found") }    
            Err(e) => { panic!("{:?}", e) },
        }
    };

    let source_ip = {
        match get_local_ip() {
            Some(ip) => {ip}
            None => panic!("Cant locate a local ip address")
        }
    };

    eth_packet.set_source(source_mac.into_MacAddr());
    eth_packet.set_ethertype(EtherTypes::Arp);

    let mut arp_buffer = [0u8; 28];
    let mut arp_packet = MutableArpPacket::new(&mut arp_buffer).unwrap();

    arp_packet.set_hardware_type(ArpHardwareTypes::Ethernet);
    arp_packet.set_protocol_type(EtherTypes::Ipv4);
    arp_packet.set_hw_addr_len(6);
    arp_packet.set_proto_addr_len(4);
    arp_packet.set_operation(ArpOperations::Request);
    arp_packet.set_sender_hw_addr(source_mac.into_MacAddr());
    arp_packet.set_sender_proto_addr(source_ip);
    arp_packet.set_target_hw_addr(MacAddr::zero());
    arp_packet.set_target_proto_addr(target_ip);

    eth_packet.set_payload(arp_packet.packet_mut());

    sender.send_to(&eth_packet.packet_mut(), Some(local_interface));
}

pub fn send_arp_res(
    target_mac: MacAddr,
    target_ip: Ipv4Addr,
) {

    let local_interface = {
        match get_local_interface() {
            Some(interface) => { interface }
            None => panic!("Can't find interface")
        }

    };

    let (mut sender, receiver) = match datalink::channel(
        &local_interface, Default::default()) {

        Ok(Channel::Ethernet(
            sender, receiver
        )) => (sender, receiver),

        Ok(_) => panic!("Unknown channel type"),
        Err(e) => panic!("Error happened {}", e),
    };

    let mut eth_buf = [0u8; 42];

    let mut eth_packet = MutableEthernetPacket::new(
        &mut eth_buf
    ).unwrap();

    
    let source_mac = {
        match get_mac_address() {
            Ok(Some(ma)) => { ma }
            Ok(None) => { panic!("Mac address wasn't found") }    
            Err(e) => { panic!("{:?}", e) },
        }
    };
    
    let source_ip = {
        match get_local_ip() {
            Some(ip) => {ip}
            None => panic!("Cant locate a local ip address")
        }
    };
    
    eth_packet.set_destination(target_mac);
    eth_packet.set_source(source_mac.into_MacAddr());
    eth_packet.set_ethertype(EtherTypes::Arp);

    let mut arp_buffer = [0u8; 28];
    let mut arp_packet = MutableArpPacket::new(&mut arp_buffer).unwrap();

    arp_packet.set_hardware_type(ArpHardwareTypes::Ethernet);
    arp_packet.set_protocol_type(EtherTypes::Ipv4);
    arp_packet.set_hw_addr_len(6);
    arp_packet.set_proto_addr_len(4);
    arp_packet.set_operation(ArpOperations::Reply);
    arp_packet.set_sender_hw_addr(source_mac.into_MacAddr());
    arp_packet.set_sender_proto_addr(Ipv4Addr::new(10, 100, 102, 1));
    arp_packet.set_target_hw_addr(target_mac);
    arp_packet.set_target_proto_addr(target_ip);

    eth_packet.set_payload(arp_packet.packet_mut());

    sender.send_to(&eth_packet.packet_mut(), Some(local_interface));
}
