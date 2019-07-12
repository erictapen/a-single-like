extern crate pcarp;
extern crate png;
extern crate svg;
#[macro_use]
extern crate serde_derive;

mod coloring;
mod column;
mod datagram;
mod cleartext_datagram;

use std::path::PathBuf;
// use std::time::{Duration, SystemTime};

use coloring::ColorArea;
use std::collections::HashMap;

fn main() {
    let colorings = {
        let mut c = HashMap::new();
        c.insert(
            1,
            vec![
                ColorArea::sent_our_ip(), 
                ColorArea::sent_their_ip(),
                ColorArea::tcp_flag(),
            ],
        );
        c.insert(
            2,
            vec![
                ColorArea::rcvd_our_ip(), 
                ColorArea::rcvd_their_ip(),
                ColorArea::tcp_flag(),
            ],
        );
        c.insert(
            3,
            vec![
                ColorArea::sent_our_ip(),
                ColorArea::sent_their_ip(),
                ColorArea::tcp_flag(),
            ],
        );
        c.insert(
            4,
            vec![
                ColorArea::sent_our_ip(),
                ColorArea::sent_their_ip(),
                ColorArea::tcp_flag(),
                // Client Hello
                ColorArea {
                    from: 0x34 * 8,
                    to: 0x274 * 8,
                    colors: coloring::TLS_STUFF,
                },
            ],
        );
        c.insert(
            5,
            vec![
                ColorArea::rcvd_our_ip(), 
                ColorArea::rcvd_their_ip(),
                ColorArea::tcp_flag(),
            ],
        );
        c.insert(
            6,
            vec![
                ColorArea::rcvd_our_ip(), 
                ColorArea::rcvd_their_ip(),
                ColorArea::tcp_flag(),
                // Client Hello
                ColorArea {
                    from: 0x34 * 8,
                    to: 0x8a * 8,
                    colors: coloring::TLS_STUFF,
                },
                // Change Cipher Spec
                ColorArea {
                    from: 0x8a * 8,
                    to: 0x90 * 8,
                    colors: coloring::TLS_STUFF,
                },
                // Handshake Finished
                ColorArea {
                    from: 0x90 * 8,
                    to: 0xbd * 8,
                    colors: coloring::TLS_STUFF,
                },
            ],
        );
        c.insert(
            7,
            vec![
                ColorArea::sent_our_ip(),
                ColorArea::sent_their_ip(),
                ColorArea::tcp_flag(),
            ],
        );
        c.insert(
            8,
            vec![
                ColorArea::sent_our_ip(),
                ColorArea::sent_their_ip(),
                ColorArea::tcp_flag(),
                // Change Cipher Spec
                ColorArea {
                    from: 0x34 * 8,
                    to: 0x3b * 8,
                    colors: coloring::TLS_STUFF,
                },
                // Handshake Finished
                ColorArea {
                    from: 0x3a * 8,
                    to: 0x67 * 8,
                    colors: coloring::TLS_STUFF,
                },
            ],
        );
        c.insert(
            9,
            vec![
                ColorArea::sent_our_ip(),
                ColorArea::sent_their_ip(),
                ColorArea::tcp_flag(),
                // Application Data
                ColorArea {
                    from: 0x34 * 8,
                    to: 0x2e7 * 8,
                    colors: coloring::TLS_STUFF,
                },
            ],
        );
        c.insert(
            10,
            vec![
                ColorArea::rcvd_our_ip(), 
                ColorArea::rcvd_their_ip(),
                ColorArea::tcp_flag(),
            ],
        );
        c.insert(
            11,
            vec![
                ColorArea::rcvd_our_ip(), 
                ColorArea::rcvd_their_ip(),
                ColorArea::tcp_flag(),
                // Application Data
                ColorArea {
                    from: 0x34 * 8,
                    to: 0x21e * 8,
                    colors: coloring::TLS_STUFF,
                },
                // Application Data
                ColorArea {
                    from: 0x21e * 8,
                    to: 0x24a * 8,
                    colors: coloring::TLS_STUFF,
                },
            ],
        );
        c.insert(
            12,
            vec![
                ColorArea::sent_our_ip(),
                ColorArea::sent_their_ip(),
                ColorArea::tcp_flag(),
            ],
        );
        c.insert(
            13,
            vec![
                ColorArea::rcvd_our_ip(), 
                ColorArea::rcvd_their_ip(),
                ColorArea::tcp_flag(),
            ],
        );
        c.insert(
            14,
            vec![
                ColorArea::sent_our_ip(),
                ColorArea::sent_their_ip(),
                ColorArea::tcp_flag(),
                // Close Notify
                ColorArea {
                    from: 0x34 * 8,
                    to: 0x53 * 8,
                    colors: coloring::TLS_STUFF,
                },
            ],
        );
        c.insert(
            15,
            vec![
                ColorArea::sent_our_ip(),
                ColorArea::sent_their_ip(),
                ColorArea::tcp_flag(),
            ],
        );
        c.insert(
            16,
            vec![
                ColorArea::rcvd_our_ip(), 
                ColorArea::rcvd_their_ip(),
                ColorArea::tcp_flag(),
            ],
        );
        c
    };
    let cleartext_colorings = {
        let mut c = HashMap::new();
        c.insert(
            9,
            vec![
                // applause-project
                ColorArea {
                    from: 13 * 8,
                    to: 29 * 8,
                    // Olive
                    colors: ([128, 128, 0], [64, 64, 0]),
                },
                // project ID 12035
                ColorArea {
                    from: 33 * 8,
                    to: 38 * 8,
                    // Orange
                    colors: ([245, 130, 48], [122, 65, 24]),
                },
                // User agent
                ColorArea {
                    from: 81 * 8,
                    to: 161 * 8,
                    // Lime
                    colors: ([210, 245, 60], [105, 122, 30]),
                },
                // language
                ColorArea {
                    from: 236 * 8,
                    to: 241 * 8,
                    // Magenta
                    colors: ([240, 50, 230], [120, 25, 115]),
                },
                // session ID
                ColorArea {
                    from: 498 * 8,
                    to: 550 * 8,
                    // Pink
                    colors: ([250, 190, 190], [125, 95, 95]),
                },
                // user ID
                ColorArea {
                    from: 577 * 8,
                    to: 609* 8,
                    // Pink
                    colors: ([250, 190, 190], [125, 95, 95]),
                },
                // session ID
                ColorArea {
                    from: 619 * 8,
                    to: 645 * 8,
                    // Pink
                    colors: ([250, 190, 190], [125, 95, 95]),
                },
            ],
        );
        c.insert(
            11,
            vec![
                // user id
                ColorArea {
                    from: 225 * 8,
                    to: 257 * 8,
                    // Pink
                    colors: ([250, 190, 190], [125, 95, 95]),
                },
                // success: true
                ColorArea {
                    from: 460 * 8,
                    to: 476 * 8,
                    // Purple
                    colors: ([145, 30, 180], [72, 15, 90]),
                },
            ],
        );
        c
    };
    let cleartext_paths = {
        let mut dp = HashMap::new();
        dp.insert(9, PathBuf::from("./raw_data/incom-blume/tls-stream-laptop-to-server.raw"));
        dp.insert(11, PathBuf::from("./raw_data/incom-blume/tls-stream-server-to-laptop.raw"));
        dp
    };
    use column::PacketType::Sent as S;
    use column::PacketType::Received as R;
    let column = column::Column {
        data_path: PathBuf::from("./raw_data/incom-blume/incom-blume_filtered.pcapng"),
        packet_types: vec![
            S,
            R,
            S,
            S,
            R,
            R,
            S,
            S,
            S,
            R,
            R,
            S,
            R,
            S,
            S,
            R,
        ],
        colorings: colorings,
        cleartext_colorings: cleartext_colorings,
        cleartext_paths: cleartext_paths,
    };
    column.svg().unwrap();
}
