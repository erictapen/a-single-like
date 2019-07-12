use pcarp::Capture;
use std::fs::File;
use std::path::PathBuf;

use crate::cleartext_datagram::CleartextDatagram;
use crate::coloring::Coloring;
use crate::datagram::Datagram;

use std::collections::HashMap;

use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Eine Spalte im Plakat, die die IP-Pakete anzeigt.
pub struct Column {
    /// pcapng file with the already filtered raw data
    pub data_path: PathBuf,
    // pub our_ip: Vec<Byte>,
    // pub their_ip: Vec<Byte>,
    pub packet_types: Vec<PacketType>,
    pub colorings: HashMap<usize, Coloring>,
    pub cleartext_colorings: HashMap<usize, Coloring>,
    pub cleartext_paths: HashMap<usize, PathBuf>,
}

pub enum PacketType {
    Sent,
    Received,
}

impl Column {
    pub fn svg(&self) -> Result<PathBuf, &str> {
        let mut datagrams: Vec<Datagram> = Vec::new();
        let file = File::open(&self.data_path).expect(&format!(
            "Could not find {}",
            &self.data_path.to_str().unwrap()
        ));
        let mut pcap = Capture::new(file).unwrap();
        // count starts at 1, as Wireshark also starts at 1…
        let mut count: usize = 1;
        while let Some(packet) = pcap.next() {
            if let Ok(packet) = packet {
                let datagram: Datagram = Datagram {
                    packet_data: packet.data.to_vec(),
                    timestamp: packet.timestamp.unwrap_or(UNIX_EPOCH),
                    gray_value: 0,
                    coloring: self.colorings.get(&count).unwrap().clone(),
                    file_name: format!("./out/packet-{}.png", count),
                };
                datagrams.push(datagram);
            };
            count += 1;
        }

        if datagrams.len() != self.packet_types.len() {
            panic!("Input File {:?} has {} packets, but packet_types got {} elements. They must be equal.",
                   self.data_path,
                   datagrams.len(),
                   self.packet_types.len(),
                   );
        }

        use svg::node::element::{Group, Image};
        use svg::Node;
        let mut bytes_ingoing: usize = 0;
        let mut bytes_outgoing: usize = 0;
        let group = {
            let mut g = Group::new();
            let mut pivot: usize = 0;

            let start_time: SystemTime =
                SystemTime::UNIX_EPOCH + Duration::new(1561975021, 890838943);
            let end_time: SystemTime =
                SystemTime::UNIX_EPOCH + Duration::new(1561975026, 999785537);

            let capture_duration: Duration = end_time.duration_since(start_time).unwrap();

            const GAP_SIZE: usize = 5;
            for i in 0..datagrams.len() {
                match self.packet_types[i] {
                    PacketType::Sent => {
                        bytes_outgoing += datagrams[i].size();
                    },
                    PacketType::Received => {
                        bytes_ingoing += datagrams[i].size();
                    },
                };
                let d = &datagrams[i];
                let x = match self.packet_types[i] {
                    PacketType::Sent => 0,
                    PacketType::Received => 10,
                };
                g.append(
                    Image::new()
                        .set("id", format!("packet-{}", i + 1))
                        .set("xlink:href", d.png().unwrap().to_str().unwrap())
                        .set("x", x)
                        .set("y", pivot.to_string())
                        .set("style", "image-rendering:optimizeSpeed"),
                );
                let i_plus_one = &i + 1;
                if self.cleartext_paths.contains_key(&i_plus_one) {
                    let ct_datagram: CleartextDatagram = CleartextDatagram {
                        data_path: self.cleartext_paths.get(&i_plus_one).unwrap().to_path_buf(),
                        coloring: self.cleartext_colorings.get(&i_plus_one).unwrap().to_vec(),
                        file_name: format!("./out/cleartext-{}.png", &i_plus_one),
                    };
                    g.append(
                        Image::new()
                            .set("id", format!("cleartext-{}", i + 1))
                            .set("xlink:href", ct_datagram.png().unwrap().to_str().unwrap())
                            .set("x", 128 + 64)
                            .set("y", (pivot + 3).to_string())
                            .set("style", "image-rendering:optimizeSpeed"),
                    );
                }
                pivot += d.height() + GAP_SIZE;
            }
            g
        };

        println!(
            "Column contains {} ingoing bytes and {} outgoing bytes.",
            bytes_ingoing, bytes_outgoing
        );

        Ok(PathBuf::from("out/column.svg"))
    }
}
