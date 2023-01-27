use std::net::IpAddr;

use pnet::{packet::{ipv4::Ipv4Packet, Packet, ipv6::Ipv6Packet, arp::ArpPacket}, util::MacAddr};
use super::{wrapers::{ParsedPacket, SerializableIpv4Packet, SerializablePacket, SerializableIpv6Packet, SerializableArpPacket}, transport::handle_transport_protocol};


pub fn handle_ipv4_packet(packet: &[u8], parsed_packet: &mut ParsedPacket) {
    let header = Ipv4Packet::new(packet);
    if let Some(header) = header {
        parsed_packet.set_network_layer_packet(Some(SerializablePacket::Ipv4Packet(
            SerializableIpv4Packet::from(&header),
        )));
        handle_transport_protocol(
            IpAddr::V4(header.get_source()),
            IpAddr::V4(header.get_destination()),
            header.get_next_level_protocol(),
            header.payload(),
            parsed_packet,
        );
    } else {
        parsed_packet.set_network_layer_packet(Some(SerializablePacket::MalformedPacket(
            "Malformed IPv4 Packet".to_string(),
        )));
    }
}

/// Build a IPv6 packet from a data-link packet, save it in a Parsed Packet
pub fn handle_ipv6_packet(packet: &[u8], parsed_packet: &mut ParsedPacket) {
    let header = Ipv6Packet::new(packet);
    if let Some(header) = header {
        parsed_packet.set_network_layer_packet(Some(SerializablePacket::Ipv6Packet(
            SerializableIpv6Packet::from(&header),
        )));
        handle_transport_protocol(
            IpAddr::V6(header.get_source()),
            IpAddr::V6(header.get_destination()),
            header.get_next_header(),
            header.payload(),
            parsed_packet,
        );
    } else {
        parsed_packet.set_network_layer_packet(Some(SerializablePacket::MalformedPacket(
            "Malformed IPv6 Packet".to_string(),
        )));
    }
}

/// Build a ARP packet from a data-link packet, save it in a Parsed Packet
pub fn handle_arp_packet(
    packet: &[u8],
    source: MacAddr,
    dest: MacAddr,
    parsed_packet: &mut ParsedPacket,
) {
    let header = ArpPacket::new(packet);
    if let Some(header) = header {
        parsed_packet.set_network_layer_packet(Some(SerializablePacket::ArpPacket(
            SerializableArpPacket::from(&header),
        )));
    } else {
        parsed_packet.set_network_layer_packet(Some(SerializablePacket::MalformedPacket(
            "Malformed ARP Packet".to_string(),
        )));
    }
}