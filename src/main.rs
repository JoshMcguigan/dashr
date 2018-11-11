extern crate pnet;

use pnet::packet::Packet;
use pnet::packet::ethernet::{EtherTypes, EthernetPacket};
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::udp::UdpPacket;
use pnet::datalink::{self, NetworkInterface, Channel::Ethernet};
use std::time::{SystemTime, Duration, UNIX_EPOCH};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    /// Network interface
    network_interface: String,

    /// Delay time (seconds) between presses
    #[structopt(short = "d", long = "delay", default_value = "5")]
    delay: u64,
}

fn main() {
    let opt = Opt::from_args();

    let interface_names_match = |iface: &NetworkInterface| iface.name == opt.network_interface;

    // Find the network interface with the provided name
    let interfaces = datalink::interfaces();
    let interface = interfaces.into_iter().filter(interface_names_match).next().unwrap();

    // Create a channel to receive on
    let (_, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("dashr: unhandled channel type"),
        Err(e) => panic!("dashr: unable to create channel: {}", e),
    };

    let mut last_trigger_time = Duration::new(0, 0);

    loop {
        match rx.next() {
            Ok(packet) => {
                handle_ethernet_frame(&EthernetPacket::new(packet).unwrap(), &mut last_trigger_time, opt.delay);
            },
            Err(e) => panic!("dashr: unable to receive packet: {}", e),
        }
    }
}

fn handle_ethernet_frame(ethernet: &EthernetPacket, last_trigger_time: &mut Duration, delay: u64) {
    if ethernet.get_ethertype() == EtherTypes::Ipv4 {
        let header = Ipv4Packet::new(ethernet.payload());
        if let Some(header) = header {
            let packet = header.payload();
            let udp = UdpPacket::new(packet);
            if let Some(udp) = udp {
                if udp.get_source() == 68 && udp.get_destination() == 67 {
                    if last_trigger_time.as_secs() + delay < get_current_time().as_secs() {
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
