use pnet::packet::{Packet, ethernet::EtherTypes};
pub mod wrapers;
pub mod network;
pub mod transport;
use wrapers::{ParsedPacket, SerializableEthernetPacket};
use pnet::{
    datalink::{
        self, interfaces, Channel::Ethernet, Config, DataLinkReceiver, DataLinkSender,
        NetworkInterface,
    },
    packet::ethernet::EthernetPacket,
};
use thiserror::Error;

use self::{wrapers::{SerializablePacket, SerializableUnknownPacket}, network::{handle_ipv4_packet, handle_ipv6_packet, handle_arp_packet}};

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("Channel could not be created")]
    ChannelCreationError,
    #[error("Channel is not supported")]
    ChannelNotSupported,
}

pub struct Parser {
    interface: NetworkInterface,
    tx: Box<dyn DataLinkSender>,
    rx: Box<dyn DataLinkReceiver>,
}

impl Parser {
    pub fn interfaces() -> Vec<NetworkInterface> {
        datalink::interfaces()
    }

    pub fn new(interface_name: String) -> Result<Self, ParserError> {
        let interfaces = datalink::interfaces();
        let interface = interfaces
            .into_iter()
            .filter(|x| x.name == interface_name)
            .next()
            .unwrap();

        let config = Config {
            // read_timeout: Some(Duration::ZERO),
            ..Default::default()
        };

        let (tx, rx) = match datalink::channel(&interface, config) {
            Ok(Ethernet(tx, rx)) => (tx, rx),
            Ok(_) => return Err(ParserError::ChannelNotSupported),
            Err(_) => return Err(ParserError::ChannelCreationError),
        };

        Ok(Self { interface, tx, rx })
    }

    pub fn next(&mut self) -> Option<&[u8]> {
        self.rx.next().ok()
    }

    /// Parse ethernet frame obtaining the packet link-layer and network-layer representations
    pub fn parse_ethernet_frame(ethernet: &EthernetPacket, id: usize) -> ParsedPacket {
        let mut parsed_packet = ParsedPacket::new(id);

        parsed_packet.set_link_layer_packet(Some(SerializablePacket::EthernetPacket(
            SerializableEthernetPacket::from(ethernet),
        )));

        match ethernet.get_ethertype() {
            EtherTypes::Ipv4 => handle_ipv4_packet(ethernet.payload(), &mut parsed_packet),
            EtherTypes::Ipv6 => handle_ipv6_packet(ethernet.payload(), &mut parsed_packet),
            EtherTypes::Arp => handle_arp_packet(
                ethernet.payload(),
                ethernet.get_source(),
                ethernet.get_destination(),
                &mut parsed_packet,
            ),
            _ => {

                parsed_packet.set_link_layer_packet(Some(SerializablePacket::UnknownPacket(
                    SerializableUnknownPacket::from(ethernet),
                )));
            }
        }

        parsed_packet
    }
}
