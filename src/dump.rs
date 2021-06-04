use pnet;
use pnet::datalink::{self, NetworkInterface};
use pnet::datalink::Channel::Ethernet;

use std::sync::mpsc::Receiver;
use std::{sync::mpsc, thread};

#[derive(Debug)]
pub enum ListenerError {
    UnhandledChannelType,
    ChannelCreationError,
    ReceiverIsNotSet,
}

pub struct Dumper {
    interface: NetworkInterface,
    read_channel: Option<Receiver<Vec<u8>>>,
    pub packets: Vec<Vec<u8>>,
}

impl Dumper {
    pub fn new(name: String) -> Option<Self> {
        let interfaces = datalink::interfaces();

        let interface = interfaces
            .into_iter()
            .filter(|iface: &NetworkInterface| iface.name == name)
            .next();

        match interface {
            Some(interface) => Some(Self {
                interface,
                read_channel: None,
                packets: Vec::new(),
            }),
            None => None,
        }
    }

    pub fn update(&mut self) -> Result<usize, ListenerError> {
        if let Some(rx) = &self.read_channel {
            let mut counter = 0;

            for item in rx.try_iter() {
                self.packets.push(item.to_owned());
                counter += 1;
            }

            Ok(counter)
        } else {
            Err(ListenerError::ReceiverIsNotSet)
        }
    }

    pub fn start_listening(&mut self) -> Result<(), ListenerError> {
        let (_, mut packet_receiver) = match datalink::channel(&self.interface, Default::default())
        {
            Ok(Ethernet(tx, rx)) => (tx, rx),
            Ok(_) => return Err(ListenerError::UnhandledChannelType),
            Err(_) => return Err(ListenerError::ChannelCreationError),
        };

        let (tx, rx) = mpsc::channel();

        self.read_channel = Some(rx);

        thread::spawn(move || loop {
            match packet_receiver.next() {
                Ok(packet) => {
                    // Panic of sending error
                    tx.send(packet.to_owned()).unwrap();
                }
                Err(_) => {}
            }
        });

        Ok(())
    }
}
