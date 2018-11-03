extern crate pnet;

use std::io::{self, Write};
use std::env;
use std::process;
use pnet::packet::Packet;
use pnet::packet::ethernet::{EtherTypes, EthernetPacket};
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::udp::UdpPacket;
use pnet::datalink::{self, NetworkInterface};
use std::time::SystemTime;
use std::time::Duration;
use std::time::UNIX_EPOCH;

fn main() {
    use pnet::datalink::Channel::Ethernet;

    let iface_name = match env::args().nth(1) {
        Some(n) => n,
        None => {
            writeln!(io::stderr(), "USAGE: dashr <NETWORK INTERFACE>").unwrap();
            process::exit(1);
        },
    };
    let interface_names_match = |iface: &NetworkInterface| iface.name == iface_name;

    // Find the network interface with the provided name
    let interfaces = datalink::interfaces();
    let interface = interfaces.into_iter().filter(interface_names_match).next().unwrap();

    // Create a channel to receive on
    let (_, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("dashr: unhandled channel type: {}"),
        Err(e) => panic!("dashr: unable to create channel: {}", e),
    };

    let mut last_trigger_time = Duration::new(0, 0);

    loop {
        match rx.next() {
            Ok(packet) => {
                handle_ethernet_frame(&EthernetPacket::new(packet).unwrap(), &mut last_trigger_time);
            },
            Err(e) => panic!("dashr: unable to receive packet: {}", e),
        }
    }
}

fn handle_ethernet_frame(ethernet: &EthernetPacket, last_trigger_time: &mut Duration) {
    if ethernet.get_ethertype() == EtherTypes::Ipv4 {
        let header = Ipv4Packet::new(ethernet.payload());
        if let Some(header) = header {
            let packet = header.payload();
            let udp = UdpPacket::new(packet);
            if let Some(udp) = udp {
                if udp.get_source() == 68 && udp.get_destination() == 67 {
                    let debouce_time = 2; // debounce time in seconds TODO this could be a command line arg
                    if last_trigger_time.as_secs() + debouce_time < get_current_time().as_secs() {
                        *last_trigger_time = get_current_time();
                        print_client_mac_from_dhcp_discovery_packet(packet);
                    }

                }
            }
        }
    }
}

fn get_current_time() -> Duration {
    SystemTime::now().duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
}

fn print_client_mac_from_dhcp_discovery_packet(packet: &[u8]){
    let mut s = Vec::new();
    for &byte in &packet[36..42] {
        s.push(format!("{:X}", byte));
    }

    println!("{}", s.join(":"));
}
