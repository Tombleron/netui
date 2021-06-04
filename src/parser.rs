use pnet::packet::arp::ArpPacket;
use pnet::packet::ethernet::{EtherTypes, EthernetPacket};
use pnet::packet::icmp::IcmpPacket;
use pnet::packet::icmpv6::Icmpv6Packet;
use pnet::packet::ip::{IpNextHeaderProtocol, IpNextHeaderProtocols};
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::ipv6::Ipv6Packet;
use pnet::packet::tcp::TcpPacket;
use pnet::packet::udp::UdpPacket;
use pnet::packet::Packet;

use std::net::IpAddr;

use crate::ui::ListItem;

pub struct Parsed {
    proto: Option<String>,
    size: Option<String>,
    from: Option<String>,
    to: Option<String>,
    other: Option<String>,
}

impl Parsed {
    pub fn parse(packet: &Vec<u8>) -> Self {
        let mut parsed_frame = Self {
            proto: None,
            from: None,
            to: None,
            size: None,
            other: None,
        };

        let eth_packet = match EthernetPacket::new(packet) {
            Some(packet) => packet,
            None => {
                return parsed_frame;
            }
        };

        parsed_frame.size = Some(format!("{}", packet.len()));
        parsed_frame.handle_ethernet_frame(&eth_packet);
        parsed_frame
    }

    fn handle_ethernet_frame(&mut self, ethernet: &EthernetPacket) {
        match ethernet.get_ethertype() {
            EtherTypes::Ipv4 => self.handle_ipv4_packet(ethernet),
            EtherTypes::Ipv6 => self.handle_ipv6_packet(ethernet),
            EtherTypes::Arp => self.handle_arp_packet(ethernet),
            _ => {}
        }
    }

    fn handle_ipv4_packet(&mut self, ethernet: &EthernetPacket) {
        let header = Ipv4Packet::new(ethernet.payload());

        if let Some(header) = header {
            self.from = Some(format!("{}", IpAddr::V4(header.get_source())));
            self.to = Some(format!("{}", IpAddr::V4(header.get_destination())));
            self.handle_transport_protocol(header.get_next_level_protocol(), header.payload());
        }
    }

    fn handle_ipv6_packet(&mut self, ethernet: &EthernetPacket) {
        let header = Ipv6Packet::new(ethernet.payload());

        if let Some(header) = header {
            self.from = Some(format!("{}", IpAddr::V6(header.get_source())));
            self.to = Some(format!("{}", IpAddr::V6(header.get_destination())));
            self.handle_transport_protocol(header.get_next_header(), header.payload());
        }
    }

    fn handle_arp_packet(&mut self, ethernet: &EthernetPacket) {
        let header = ArpPacket::new(ethernet.payload());

        if let Some(header) = header {
            self.proto = Some("ARP".to_string());
            self.other = Some(format!("{:?}", header.get_operation()));
            self.from = Some(format!("{}", ethernet.get_source()));
            self.to = Some(format!("{}", ethernet.get_destination()));
        }
    }

    fn handle_transport_protocol(&mut self, protocol: IpNextHeaderProtocol, packet: &[u8]) {
        match protocol {
            IpNextHeaderProtocols::Udp => self.handle_udp_packet(packet),
            IpNextHeaderProtocols::Tcp => self.handle_tcp_packet(packet),
            IpNextHeaderProtocols::Icmp => self.handle_icmp_packet(packet),
            IpNextHeaderProtocols::Icmpv6 => self.handle_icmpv6_packet(packet),
            _ => {}
        }
    }

    fn handle_udp_packet(&mut self, packet: &[u8]) {
        let udp = UdpPacket::new(packet);

        if udp.is_some() {
            self.proto = Some("UDP".to_string());
        }
    }

    fn handle_tcp_packet(&mut self, packet: &[u8]) {
        let tcp = TcpPacket::new(packet);

        if tcp.is_some() {
            self.proto = Some("TCP".to_string());
        }
    }

    fn handle_icmp_packet(&mut self, packet: &[u8]) {
        let icmp_packet = IcmpPacket::new(packet);

        if let Some(icmp_packet) = icmp_packet {
            self.proto = Some("ICMP".to_string());
            self.other = Some(format!("{:?}", icmp_packet.get_icmp_type()));
        }
    }

    fn handle_icmpv6_packet(&mut self, packet: &[u8]) {
        let icmpv6_packet = Icmpv6Packet::new(packet);

        if let Some(icmpv6_packet) = icmpv6_packet {
            self.proto = Some("ICMPv6".to_string());
            self.other = Some(format!("{:?}", icmpv6_packet.get_icmpv6_type()));
        }
    }
}

impl Into<ListItem> for Parsed {
    fn into(self) -> ListItem {
        let mut elements = Vec::new();

        elements.push(self.proto.unwrap_or("-".to_string()));
        elements.push(format!(
            "{} -> {}",
            self.from.unwrap_or("-".to_string()),
            self.to.unwrap_or("-".to_string())
        ));
        elements.push(self.size.unwrap_or("-".to_string()));
        elements.push(self.other.unwrap_or("-".to_string()));

        ListItem { elements }
    }
}
