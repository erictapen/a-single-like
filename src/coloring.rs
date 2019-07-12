#[derive(Hash, Clone)]
pub struct ColorArea {
    pub from: usize,
    pub to: usize,
    // 0,0,0 is going to be ignored
    pub colors: ([u8; 3], [u8; 3]),
}

pub type Coloring = Vec<ColorArea>;

// Green
const OUR_IP: ([u8; 3], [u8; 3]) = ([60, 180, 75], [30, 90, 37]);
// Blue
const THEIR_IP: ([u8; 3], [u8; 3]) = ([0, 130, 200], [0, 65, 100]);

// Lime
const OUR_TCP_PORT: ([u8; 3], [u8; 3]) = ([210, 245, 60], [105, 122, 30]);
// Cyan
const THEIR_TCP_PORT: ([u8; 3], [u8; 3]) = ([70, 240, 240], [35, 120, 120]);

// Yellow
const TCP_FLAG: ([u8; 3], [u8; 3]) = ([255, 255, 25], [128, 128, 12]);

// Mint
pub const TLS_STUFF: ([u8; 3], [u8; 3]) = ([170, 255, 195], [85, 128, 98]);

impl ColorArea {
    pub fn sent_our_ip() -> ColorArea {
        ColorArea {
            from: 12 * 8,
            to: 16 * 8,
            colors: OUR_IP,
        }
    }
    pub fn sent_their_ip() -> ColorArea {
        ColorArea {
            from: 16 * 8,
            to: 20 * 8,
            colors: THEIR_IP,
        }
    }
    pub fn rcvd_our_ip() -> ColorArea {
        ColorArea {
            from: 16 * 8,
            to: 20 * 8,
            colors: OUR_IP,
        }
    }
    pub fn rcvd_their_ip() -> ColorArea {
        ColorArea {
            from: 12 * 8,
            to: 16 * 8,
            colors: THEIR_IP,
        }
    }

    pub fn sent_our_tcp_port() -> ColorArea {
        ColorArea {
            from: 20 * 8,
            to: 22 * 8,
            colors: OUR_TCP_PORT,
        }
    }
    pub fn sent_their_tcp_port() -> ColorArea {
        ColorArea {
            from: 22 * 8,
            to: 24 * 8,
            colors: THEIR_TCP_PORT,
        }
    }
    pub fn rcvd_our_tcp_port() -> ColorArea {
        ColorArea {
            from: 22 * 8,
            to: 24 * 8,
            colors: OUR_TCP_PORT,
        }
    }
    pub fn rcvd_their_tcp_port() -> ColorArea {
        ColorArea {
            from: 20 * 8,
            to: 22 * 8,
            colors: THEIR_TCP_PORT,
        }
    }

    pub fn tcp_flag() -> ColorArea {
        ColorArea {
            from: 33 * 8,
            to: 34 * 8,
            colors: TCP_FLAG,
        }
    }
}
