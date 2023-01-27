use std::net::IpAddr;

use pnet::packet::{
    ip::{IpNextHeaderProtocol, IpNextHeaderProtocols},
    tcp::TcpPacket,
    udp::UdpPacket,
    Packet, icmp::{IcmpPacket, IcmpTypes, echo_reply, echo_request}, icmpv6::Icmpv6Packet,
};

use super::wrapers::{
    ParsedPacket, SerializablePacket, SerializableTcpPacket, SerializableUdpPacket, SerializableEchoReplyPacket, SerializableEchoRequestPacket, SerializableIcmpPacket, SerializableIcmpv6Packet,
};

pub fn handle_udp_packet(
    source: IpAddr,
    destination: IpAddr,
    packet: &[u8],
    parsed_packet: &mut ParsedPacket,
) {
    let udp = UdpPacket::new(packet);

    if let Some(udp) = udp {
        parsed_packet.set_transport_layer_packet(Some(SerializablePacket::UdpPacket(
            SerializableUdpPacket::from(&udp),
        )));

        // handle_application_protocol(
        //     source,
        //     udp.get_source(),
        //     destination,
        //     udp.get_destination(),
        //     false,
        //     udp.payload(),
        //     parsed_packet,
        // );
    } else {
        parsed_packet.set_transport_layer_packet(Some(SerializablePacket::MalformedPacket(
            "Malformed UDP Packet".to_string(),
        )));
    }
}

const ACK_BIT_SHIFT: usize = 4;
const FIN_BIT_SHIFT: usize = 0;

/// Build a TCP packet from a network-layer packet, save it in a Parsed Packet
pub fn handle_tcp_packet(
    source: IpAddr,
    destination: IpAddr,
    packet: &[u8],
    parsed_packet: &mut ParsedPacket,
) {
    let tcp = TcpPacket::new(packet);
    if let Some(tcp) = tcp {
        parsed_packet.set_transport_layer_packet(Some(SerializablePacket::TcpPacket(
            SerializableTcpPacket::from(&tcp),
        )));

        // let flags = tcp.get_flags();
        // let is_fin = (flags & (1 << ACK_BIT_SHIFT)) != 0 && (flags & (1 << FIN_BIT_SHIFT)) != 0;

        // handle_application_protocol(
        //     source,
        //     tcp.get_source(),
        //     destination,
        //     tcp.get_destination(),
        //     is_fin,
        //     tcp.payload(),
        //     parsed_packet,
        // );
    } else {
        parsed_packet.set_transport_layer_packet(Some(SerializablePacket::MalformedPacket(
            "Malformed TCP Packet".to_string(),
        )));
    }
}

/// Build a Transport-layer packet from a network-layer packet, save it in a Parsed Packet
pub fn handle_transport_protocol(
    source: IpAddr,
    destination: IpAddr,
    protocol: IpNextHeaderProtocol,
    packet: &[u8],
    parsed_packet: &mut ParsedPacket,
) {
    return match protocol {
        IpNextHeaderProtocols::Udp => handle_udp_packet(source, destination, packet, parsed_packet),
        IpNextHeaderProtocols::Tcp => handle_tcp_packet(source, destination, packet, parsed_packet),
        IpNextHeaderProtocols::Icmp => {
            handle_icmp_packet(source, destination, packet, parsed_packet)
        }
        IpNextHeaderProtocols::Icmpv6 => {
            handle_icmpv6_packet(source, destination, packet, parsed_packet)
        }
        _ => {}
    };
}

/// Build a ICMP packet from a network-layer packet, save it in a Parsed Packet
pub fn handle_icmp_packet(
    source: IpAddr,
    destination: IpAddr,
    packet: &[u8],
    parsed_packet: &mut ParsedPacket,
) {
    let icmp_packet = IcmpPacket::new(packet);
    if let Some(icmp_packet) = icmp_packet {
        match icmp_packet.get_icmp_type() {
            IcmpTypes::EchoReply => {
                let echo_reply_packet = echo_reply::EchoReplyPacket::new(packet).unwrap();

                parsed_packet.set_transport_layer_packet(Some(
                    SerializablePacket::EchoReplyPacket(SerializableEchoReplyPacket::from(
                        &echo_reply_packet,
                    )),
                ));
            }
            IcmpTypes::EchoRequest => {
                let echo_request_packet = echo_request::EchoRequestPacket::new(packet).unwrap();

                parsed_packet.set_transport_layer_packet(Some(
                    SerializablePacket::EchoRequestPacket(SerializableEchoRequestPacket::from(
                        &echo_request_packet,
                    )),
                ));
            }
            _ => {
                parsed_packet.set_transport_layer_packet(Some(SerializablePacket::IcmpPacket(
                    SerializableIcmpPacket::from(&icmp_packet),
                )));
            }
        }
    } else {
        parsed_packet.set_transport_layer_packet(Some(SerializablePacket::MalformedPacket(
            "Malformed ICMP Packet".to_string(),
        )));
    }
}

/// Build a ICMPv6 packet from a network-layer packet, save it in a Parsed Packet
pub fn handle_icmpv6_packet(
    source: IpAddr,
    destination: IpAddr,
    packet: &[u8],
    parsed_packet: &mut ParsedPacket,
) {
    let icmpv6_packet = Icmpv6Packet::new(packet);
    if let Some(icmpv6_packet) = icmpv6_packet {

        parsed_packet.set_transport_layer_packet(Some(SerializablePacket::Icmpv6Packet(
            SerializableIcmpv6Packet::from(&icmpv6_packet),
        )));
    } else {
        parsed_packet.set_transport_layer_packet(Some(SerializablePacket::MalformedPacket(
            "Malformed ICMPv6 Packet".to_string(),
        )));
    }
}
