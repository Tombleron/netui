mod network;
mod transport;
mod application;
pub use network::*;
pub use transport::*;
// use application::*;

use pnet::{util::MacAddr, packet::{ethernet::EthernetPacket, Packet}};


pub struct ParsedPacket {
    id: usize,
    link_layer_packet: Option<SerializablePacket>,
    network_layer_packet: Option<SerializablePacket>,
    transport_layer_packet: Option<SerializablePacket>,
    application_layer_packet: Option<SerializablePacket>,
}

impl ParsedPacket {
    pub fn new(id: usize) -> Self {
        ParsedPacket {
            id,
            link_layer_packet: None,
            network_layer_packet: None,
            transport_layer_packet: None,
            application_layer_packet: None,
        }
    }

    /// Get packet unique Identifier
    pub fn get_id(&self) -> usize {
        self.id
    }

    /// Get link layer packet representation
    pub fn get_link_layer_packet(&self) -> Option<&SerializablePacket> {
        self.link_layer_packet.as_ref()
    }

    /// Get network layer packet representation
    pub fn get_network_layer_packet(&self) -> Option<&SerializablePacket> {
        self.network_layer_packet.as_ref()
    }

    /// Get transport layer packet representation
    pub fn get_transport_layer_packet(&self) -> Option<&SerializablePacket> {
        self.transport_layer_packet.as_ref()
    }

    /// Get application layer packet representation
    pub fn get_application_layer_packet(&self) -> Option<&SerializablePacket> {
        self.application_layer_packet.as_ref()
    }

    /// Set link layer packet representation
    pub fn set_link_layer_packet(&mut self, link_layer_packet: Option<SerializablePacket>) {
        self.link_layer_packet = link_layer_packet;
    }

    /// Set network layer packet representation
    pub fn set_network_layer_packet(&mut self, network_layer_packet: Option<SerializablePacket>) {
        self.network_layer_packet = network_layer_packet;
    }

    /// Set transport layer packet representation
    pub fn set_transport_layer_packet(
        &mut self,
        transport_layer_packet: Option<SerializablePacket>,
    ) {
        self.transport_layer_packet = transport_layer_packet;
    }

    /// Set application layer packet representation
    pub fn set_application_layer_packet(
        &mut self,
        application_layer_packet: Option<SerializablePacket>,
    ) {
        self.application_layer_packet = application_layer_packet;
    }
}

pub enum SerializablePacket {
    EthernetPacket(SerializableEthernetPacket),
    ArpPacket(SerializableArpPacket),
    Ipv4Packet(SerializableIpv4Packet),
    Ipv6Packet(SerializableIpv6Packet),
    EchoReplyPacket(SerializableEchoReplyPacket),
    EchoRequestPacket(SerializableEchoRequestPacket),
    IcmpPacket(SerializableIcmpPacket),
    Icmpv6Packet(SerializableIcmpv6Packet),
    TcpPacket(SerializableTcpPacket),
    UdpPacket(SerializableUdpPacket),
    // HttpRequestPacket(SerializableHttpRequestPacket),
    // HttpResponsePacket(SerializableHttpResponsePacket),
    // TlsPacket(SerializableTlsPacket),
    // DnsPacket(SerializableDnsPacket),

    MalformedPacket(String),
    UnknownPacket(SerializableUnknownPacket),
}

pub struct SerializableEthernetPacket {
    pub destination: MacAddr,
    pub source: MacAddr,
    pub ethertype: String,
    pub payload: Vec<u8>,
}


impl<'a> From<&EthernetPacket<'a>> for SerializableEthernetPacket {
    fn from(packet: &EthernetPacket<'a>) -> Self {
        SerializableEthernetPacket {
            destination: packet.get_destination(),
            source: packet.get_source(),
            ethertype: packet.get_ethertype().to_string(),
            payload: packet.payload().to_vec(),
        }
    }
}

pub struct SerializableUnknownPacket {
    pub destination: MacAddr,
    pub source: MacAddr,
    pub ethertype: String,
    pub length: usize,
}

impl<'a> From<&EthernetPacket<'a>> for SerializableUnknownPacket {
    fn from(packet: &EthernetPacket<'a>) -> Self {
        SerializableUnknownPacket {
            destination: packet.get_destination(),
            source: packet.get_source(),
            ethertype: packet.get_ethertype().to_string(),
            length: packet.packet().len(),
        }
    }
}