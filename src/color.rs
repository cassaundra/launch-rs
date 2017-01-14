#[derive(Debug, Copy, Clone)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}


impl Color {
    pub fn nearest_pallette(red: u8, green: u8, blue: u8) -> u8 {
        let mut ifurthest = 0usize;
        let mut furthest = 50000;
        for (i, color) in COLOR_PALLETTE.iter().enumerate() {
            // Exact match
            if red == color.red && green == color.green && blue == color.blue {
                return i as u8;
            }

            let distance = ((red as i16 - color.red as i16).pow(2) +
                           (green as i16 - color.green as i16).pow(2) +
                           (blue as i16 - color.blue as i16).pow(2)) as u32;
            if distance < furthest {
                furthest = distance;
                ifurthest = i;
            }
        }

        return ifurthest as u8;
    }
}

// Table information from http://launchpaddr.com/mk2palette/
const COLOR_PALLETTE: [Color; 128] = [
    // 0..64
    Color{red: 0x00, green: 0x00, blue: 0x00},
    Color{red: 0x1c, green: 0x1c, blue: 0x1c},
    Color{red: 0x7c, green: 0x7c, blue: 0x7c},
    Color{red: 0xfc, green: 0xfc, blue: 0xfc},
    Color{red: 0xff, green: 0x4e, blue: 0x48},
    Color{red: 0xfe, green: 0x0a, blue: 0x00},
    Color{red: 0x5a, green: 0x00, blue: 0x00},
    Color{red: 0x18, green: 0x00, blue: 0x02},
    Color{red: 0xff, green: 0xbc, blue: 0x63},
    Color{red: 0xff, green: 0x57, blue: 0x00},
    Color{red: 0x5a, green: 0x1d, blue: 0x00},
    Color{red: 0x24, green: 0x18, blue: 0x02},
    Color{red: 0xfd, green: 0xfd, blue: 0x21},
    Color{red: 0xfd, green: 0xfd, blue: 0x00},
    Color{red: 0x58, green: 0x58, blue: 0x00},
    Color{red: 0x18, green: 0x18, blue: 0x00},
    Color{red: 0x81, green: 0xfd, blue: 0x2b},
    Color{red: 0x40, green: 0xfd, blue: 0x01},
    Color{red: 0x16, green: 0x58, blue: 0x00},
    Color{red: 0x13, green: 0x28, blue: 0x01},
    Color{red: 0x35, green: 0xfd, blue: 0x2b},
    Color{red: 0x00, green: 0xfe, blue: 0x00},
    Color{red: 0x00, green: 0x58, blue: 0x01},
    Color{red: 0x00, green: 0x18, blue: 0x00},
    Color{red: 0x35, green: 0xfc, blue: 0x47},
    Color{red: 0x00, green: 0xfe, blue: 0x00},
    Color{red: 0x00, green: 0x58, blue: 0x01},
    Color{red: 0x00, green: 0x18, blue: 0x00},
    Color{red: 0x32, green: 0xfd, blue: 0x7f},
    Color{red: 0x00, green: 0xfd, blue: 0x3a},
    Color{red: 0x01, green: 0x58, blue: 0x14},
    Color{red: 0x00, green: 0x1c, blue: 0x0e},
    Color{red: 0x2f, green: 0xfc, blue: 0xb1},
    Color{red: 0x00, green: 0xfb, blue: 0x91},
    Color{red: 0x01, green: 0x57, blue: 0x32},
    Color{red: 0x01, green: 0x18, blue: 0x10},
    Color{red: 0x39, green: 0xbe, blue: 0xff},
    Color{red: 0x00, green: 0xa7, blue: 0xff},
    Color{red: 0x01, green: 0x40, blue: 0x51},
    Color{red: 0x00, green: 0x10, blue: 0x18},
    Color{red: 0x41, green: 0x86, blue: 0xff},
    Color{red: 0x00, green: 0x50, blue: 0xff},
    Color{red: 0x01, green: 0x1a, blue: 0x5a},
    Color{red: 0x01, green: 0x06, blue: 0x19},
    Color{red: 0x47, green: 0x47, blue: 0xff},
    Color{red: 0x00, green: 0x00, blue: 0xfe},
    Color{red: 0x00, green: 0x00, blue: 0x5a},
    Color{red: 0x00, green: 0x00, blue: 0x18},
    Color{red: 0x83, green: 0x47, blue: 0xff},
    Color{red: 0x50, green: 0x00, blue: 0xff},
    Color{red: 0x16, green: 0x00, blue: 0x67},
    Color{red: 0x0a, green: 0x00, blue: 0x32},
    Color{red: 0xff, green: 0x48, blue: 0xfe},
    Color{red: 0xff, green: 0x00, blue: 0xfe},
    Color{red: 0x5a, green: 0x00, blue: 0x5a},
    Color{red: 0x18, green: 0x00, blue: 0x18},
    Color{red: 0xfb, green: 0x4e, blue: 0x83},
    Color{red: 0xff, green: 0x07, blue: 0x53},
    Color{red: 0x5a, green: 0x02, blue: 0x1b},
    Color{red: 0x21, green: 0x01, blue: 0x10},
    Color{red: 0xff, green: 0x19, blue: 0x01},
    Color{red: 0x9a, green: 0x35, blue: 0x00},
    Color{red: 0x7a, green: 0x51, blue: 0x01},
    Color{red: 0x3e, green: 0x65, blue: 0x00},

    // 64..128
    Color{red: 0x01, green: 0x38, blue: 0x00},
    Color{red: 0x00, green: 0x54, blue: 0x32},
    Color{red: 0x00, green: 0x53, blue: 0x7f},
    Color{red: 0x00, green: 0x00, blue: 0xfe},
    Color{red: 0x01, green: 0x44, blue: 0x4d},
    Color{red: 0x1a, green: 0x00, blue: 0xd1},
    Color{red: 0x7c, green: 0x7c, blue: 0x7c},
    Color{red: 0x20, green: 0x20, blue: 0x20},
    Color{red: 0xff, green: 0x0a, blue: 0x00},
    Color{red: 0xba, green: 0xfd, blue: 0x00},
    Color{red: 0xac, green: 0xec, blue: 0x00},
    Color{red: 0x56, green: 0xfd, blue: 0x00},
    Color{red: 0x00, green: 0x88, blue: 0x00},
    Color{red: 0x01, green: 0xfc, blue: 0x7b},
    Color{red: 0x00, green: 0xa7, blue: 0xff},
    Color{red: 0x02, green: 0x1a, blue: 0xff},
    Color{red: 0x35, green: 0x00, blue: 0xff},
    Color{red: 0x78, green: 0x00, blue: 0xff},
    Color{red: 0xb4, green: 0x17, blue: 0x7e},
    Color{red: 0x41, green: 0x20, blue: 0x00},
    Color{red: 0xff, green: 0x4a, blue: 0x01},
    Color{red: 0x82, green: 0xe1, blue: 0x00},
    Color{red: 0x66, green: 0xfd, blue: 0x00},
    Color{red: 0x00, green: 0xfe, blue: 0x00},
    Color{red: 0x00, green: 0xfe, blue: 0x00},
    Color{red: 0x45, green: 0xfd, blue: 0x61},
    Color{red: 0x01, green: 0xfb, blue: 0xcb},
    Color{red: 0x50, green: 0x86, blue: 0xff},
    Color{red: 0x27, green: 0x4d, blue: 0xc8},
    Color{red: 0x84, green: 0x7a, blue: 0xed},
    Color{red: 0xd3, green: 0x0c, blue: 0xff},
    Color{red: 0xff, green: 0x06, blue: 0x5a},
    Color{red: 0xff, green: 0x7d, blue: 0x01},
    Color{red: 0xb8, green: 0xb1, blue: 0x00},
    Color{red: 0x8a, green: 0xfd, blue: 0x00},
    Color{red: 0x81, green: 0x5d, blue: 0x00},
    Color{red: 0x3a, green: 0x28, blue: 0x02},
    Color{red: 0x0d, green: 0x4c, blue: 0x05},
    Color{red: 0x00, green: 0x50, blue: 0x37},
    Color{red: 0x13, green: 0x14, blue: 0x29},
    Color{red: 0x10, green: 0x1f, blue: 0x5a},
    Color{red: 0x6a, green: 0x3c, blue: 0x18},
    Color{red: 0xac, green: 0x04, blue: 0x01},
    Color{red: 0xe1, green: 0x51, blue: 0x36},
    Color{red: 0xdc, green: 0x69, blue: 0x00},
    Color{red: 0xfe, green: 0xe1, blue: 0x00},
    Color{red: 0x99, green: 0xe1, blue: 0x01},
    Color{red: 0x60, green: 0xb5, blue: 0x00},
    Color{red: 0x1b, green: 0x1c, blue: 0x31},
    Color{red: 0xdc, green: 0xfd, blue: 0x54},
    Color{red: 0x76, green: 0xfb, blue: 0xb9},
    Color{red: 0x96, green: 0x98, blue: 0xff},
    Color{red: 0x8b, green: 0x62, blue: 0xff},
    Color{red: 0x40, green: 0x40, blue: 0x40},
    Color{red: 0x74, green: 0x74, blue: 0x74},
    Color{red: 0xde, green: 0xfc, blue: 0xfc},
    Color{red: 0xa2, green: 0x04, blue: 0x01},
    Color{red: 0x34, green: 0x01, blue: 0x00},
    Color{red: 0x00, green: 0xd2, blue: 0x01},
    Color{red: 0x00, green: 0x41, blue: 0x01},
    Color{red: 0xb8, green: 0xb1, blue: 0x00},
    Color{red: 0x3c, green: 0x30, blue: 0x00},
    Color{red: 0xb4, green: 0x5d, blue: 0x00},
    Color{red: 0x4c, green: 0x13, blue: 0x00},
];