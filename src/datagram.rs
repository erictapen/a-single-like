use std::fs::File;

// For reading and opening files
use std::io::BufWriter;
use std::path::PathBuf;
// To use encoder.set()
use png::HasParameters;

use std::time::{SystemTime, UNIX_EPOCH};

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::coloring::Coloring;

pub const LINE_WIDTH: usize = 12;
pub const DEFAULT_COLOR_LOW: [u8; 3] = [0, 0, 0];
pub const DEFAULT_COLOR_HIGH: [u8; 3] = [220, 220, 220];

/// The captured packets have a fake ethernet header at the front. We don't want that, so we cut
/// off the first x bytes.
const PACKETDATA_OFFSET: usize = 16;

#[derive(Hash)]
pub struct Datagram {
    pub packet_data: Vec<u8>,
    pub timestamp: SystemTime,
    pub gray_value: u8,
    pub coloring: Coloring,
    pub file_name: String,
}

impl Datagram {
    pub fn png(&self) -> Result<PathBuf, &str> {
        let lines: usize = self.height();
        let pixel: usize = lines * LINE_WIDTH * 8;
        println!("{} lines, {} pixel", lines, pixel);

        // color data, 0,0,0 means no coloring
        let mut colors: Vec<([u8; 3], [u8; 3])> =
            vec![(DEFAULT_COLOR_LOW, DEFAULT_COLOR_HIGH); pixel];

        {
            let max = colors.len() * 4;
            for ca in &self.coloring {
                if ca.from < max && ca.to < max {
                    println!("From {} to {}", ca.from, ca.to);
                    for i in ca.from..ca.to {
                        colors[i].0 = ca.colors.0;
                        colors[i].1 = ca.colors.1;
                    }
                }
            }
        }

        // actual image data
        let mut image: Vec<u8> = vec![0; pixel * 4];

        // build image
        for i in 0..(self.packet_data.len() - PACKETDATA_OFFSET) {
            for j in 0..8 {
                let bit_i = (i * 8) + j; // bit-wise index
                for rgb_offset in 0..3 {
                    if self.packet_data[i + PACKETDATA_OFFSET] & (1 << 7 - j) == 0 {
                        // bit is a 0, use low color
                        image[bit_i * 4 + rgb_offset] = colors[bit_i].0[rgb_offset];
                    } else {
                        // bit is a 1, use high color
                        image[bit_i * 4 + rgb_offset] = colors[bit_i].1[rgb_offset];
                    }
                }
                // set alpha value to opaque
                image[bit_i * 4 + 3] = 255;
            }
        }

        println!("Datagram will have {} lines", lines);

        let path = PathBuf::from(&self.file_name);
        let file = File::create(&path).unwrap();
        let ref mut w = BufWriter::new(file);

        let mut encoder = png::Encoder::new(w, LINE_WIDTH as u32 * 8, lines as u32);
        // Width is 2 pixels and height is 1.
        encoder.set(png::ColorType::RGBA).set(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();
        writer.write_image_data(&image).unwrap(); // Save

        Ok(path)
    }

    pub fn height(&self) -> usize {
        ((self.packet_data.len() - PACKETDATA_OFFSET) / LINE_WIDTH) + 1
    }

    pub fn size(&self) -> usize {
        self.packet_data.len()
    }
}
