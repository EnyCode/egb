/*use std::{fs::File, io::Write};

#[rustfmt::skip]
const DATA: &[u8] = &[
    0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    0b00000000, 0b01000000, 0b10100000, 0b10100000, 0b11100000, 0b10100000, 0b11000000, 0b01000000, 0b01000000, 0b01000000, 0b10100000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00100000,
    0b00000000, 0b01000000, 0b10100000, 0b11100000, 0b11000000, 0b00100000, 0b11000000, 0b10000000, 0b10000000, 0b00100000, 0b01000000, 0b01000000, 0b00000000, 0b00000000, 0b00000000, 0b01000000,
    0b00000000, 0b01000000, 0b00000000, 0b10100000, 0b01100000, 0b01000000, 0b11000000, 0b00000000, 0b10000000, 0b00100000, 0b11100000, 0b11100000, 0b00000000, 0b11100000, 0b00000000, 0b01000000,
    0b00000000, 0b00000000, 0b00000000, 0b11100000, 0b11100000, 0b10000000, 0b10100000, 0b00000000, 0b10000000, 0b00100000, 0b01000000, 0b01000000, 0b01000000, 0b00000000, 0b00000000, 0b01000000,
    0b00000000, 0b01000000, 0b00000000, 0b10100000, 0b01000000, 0b10100000, 0b11100000, 0b00000000, 0b01000000, 0b01000000, 0b10100000, 0b00000000, 0b10000000, 0b00000000, 0b01000000, 0b10000000,

    0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    0b11100000, 0b11000000, 0b11100000, 0b11100000, 0b10100000, 0b11100000, 0b10000000, 0b11100000, 0b11100000, 0b11100000, 0b00000000, 0b00000000, 0b00100000, 0b00000000, 0b10000000, 0b11100000,
    0b10100000, 0b01000000, 0b00100000, 0b00100000, 0b10100000, 0b10000000, 0b10000000, 0b00100000, 0b10100000, 0b10100000, 0b01000000, 0b01000000, 0b01000000, 0b11100000, 0b01000000, 0b00100000,
    0b10100000, 0b01000000, 0b11100000, 0b01100000, 0b11100000, 0b11100000, 0b11100000, 0b00100000, 0b11100000, 0b11100000, 0b00000000, 0b00000000, 0b10000000, 0b00000000, 0b00100000, 0b01100000,
    0b10100000, 0b01000000, 0b10000000, 0b00100000, 0b00100000, 0b00100000, 0b10100000, 0b00100000, 0b10100000, 0b00100000, 0b01000000, 0b01000000, 0b01000000, 0b11100000, 0b01000000, 0b00000000,
    0b11100000, 0b11100000, 0b11100000, 0b11100000, 0b00100000, 0b11100000, 0b11100000, 0b00100000, 0b11100000, 0b00100000, 0b00000000, 0b10000000, 0b00100000, 0b00000000, 0b10000000, 0b01000000,

    0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    0b01000000, 0b11100000, 0b11100000, 0b01100000, 0b11000000, 0b11100000, 0b11100000, 0b01100000, 0b10100000, 0b11100000, 0b11100000, 0b10100000, 0b10000000, 0b11100000, 0b11000000, 0b01100000,
    0b10100000, 0b10100000, 0b10100000, 0b10000000, 0b10100000, 0b10000000, 0b10000000, 0b10000000, 0b10100000, 0b01000000, 0b01000000, 0b10100000, 0b10000000, 0b11100000, 0b10100000, 0b10100000,
    0b10100000, 0b11100000, 0b11000000, 0b10000000, 0b10100000, 0b11000000, 0b11000000, 0b10000000, 0b11100000, 0b01000000, 0b01000000, 0b11000000, 0b10000000, 0b10100000, 0b10100000, 0b10100000,
    0b10000000, 0b10100000, 0b10100000, 0b10000000, 0b10100000, 0b10000000, 0b10000000, 0b10100000, 0b10100000, 0b01000000, 0b01000000, 0b10100000, 0b10000000, 0b10100000, 0b10100000, 0b10100000,
    0b01100000, 0b10100000, 0b11100000, 0b01100000, 0b11100000, 0b11100000, 0b10000000, 0b11100000, 0b10100000, 0b11100000, 0b11000000, 0b10100000, 0b11100000, 0b10100000, 0b10100000, 0b11000000,

    0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    0b11100000, 0b01000000, 0b11100000, 0b01100000, 0b11100000, 0b10100000, 0b10100000, 0b10100000, 0b10100000, 0b10100000, 0b11100000, 0b11000000, 0b10000000, 0b01100000, 0b01000000, 0b00000000,
    0b10100000, 0b10100000, 0b10100000, 0b10000000, 0b01000000, 0b10100000, 0b10100000, 0b10100000, 0b10100000, 0b10100000, 0b00100000, 0b10000000, 0b01000000, 0b00100000, 0b10100000, 0b00000000,
    0b11100000, 0b10100000, 0b11000000, 0b11100000, 0b01000000, 0b10100000, 0b10100000, 0b10100000, 0b01000000, 0b11100000, 0b01000000, 0b10000000, 0b01000000, 0b00100000, 0b00000000, 0b00000000,
    0b10000000, 0b11000000, 0b10100000, 0b00100000, 0b01000000, 0b10100000, 0b11100000, 0b11100000, 0b10100000, 0b00100000, 0b10000000, 0b10000000, 0b01000000, 0b00100000, 0b00000000, 0b00000000,
    0b10000000, 0b01100000, 0b10100000, 0b11000000, 0b01000000, 0b01100000, 0b01000000, 0b11100000, 0b10100000, 0b11100000, 0b11100000, 0b11000000, 0b00100000, 0b01100000, 0b00000000, 0b11100000,

    0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    0b01000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    0b00100000, 0b11100000, 0b11000000, 0b11100000, 0b11000000, 0b11100000, 0b11100000, 0b11100000, 0b10100000, 0b11100000, 0b11100000, 0b10100000, 0b10000000, 0b11100000, 0b11000000, 0b01100000,
    0b00000000, 0b10100000, 0b11000000, 0b10000000, 0b10100000, 0b11000000, 0b11000000, 0b10000000, 0b10100000, 0b01000000, 0b01000000, 0b11000000, 0b10000000, 0b11100000, 0b10100000, 0b10100000,
    0b00000000, 0b11100000, 0b10100000, 0b10000000, 0b10100000, 0b10000000, 0b10000000, 0b10100000, 0b11100000, 0b01000000, 0b01000000, 0b10100000, 0b10000000, 0b10100000, 0b10100000, 0b10100000,
    0b00000000, 0b10100000, 0b11100000, 0b11100000, 0b11000000, 0b11100000, 0b10000000, 0b11100000, 0b10100000, 0b11100000, 0b11000000, 0b10100000, 0b11100000, 0b10100000, 0b10100000, 0b11000000,

    0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b01100000, 0b01000000, 0b11000000, 0b00000000, 0b00000000,
    0b11100000, 0b01000000, 0b11100000, 0b01100000, 0b11100000, 0b10100000, 0b10100000, 0b10100000, 0b10100000, 0b10100000, 0b11100000, 0b01000000, 0b01000000, 0b01000000, 0b00100000, 0b01000000,
    0b10100000, 0b10100000, 0b10100000, 0b10000000, 0b01000000, 0b10100000, 0b10100000, 0b10100000, 0b01000000, 0b11100000, 0b00100000, 0b11000000, 0b01000000, 0b01100000, 0b11100000, 0b10100000,
    0b11100000, 0b11000000, 0b11000000, 0b00100000, 0b01000000, 0b10100000, 0b11100000, 0b11100000, 0b10100000, 0b00100000, 0b10000000, 0b01000000, 0b01000000, 0b01000000, 0b10000000, 0b10100000,
    0b10000000, 0b01100000, 0b10100000, 0b11000000, 0b01000000, 0b01100000, 0b01000000, 0b11100000, 0b10100000, 0b11100000, 0b11100000, 0b01100000, 0b01000000, 0b11000000, 0b00000000, 0b11100000,

    0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    0b11111110, 0b10101010, 0b10000000, 0b01111100, 0b10001000, 0b00100000, 0b00111000, 0b01011100, 0b00111000, 0b00111000, 0b00111000, 0b01111100, 0b11111110, 0b00011100, 0b01111100, 0b00010000,
    0b11111110, 0b01010100, 0b11111110, 0b11000110, 0b00100010, 0b00111100, 0b01110100, 0b01111100, 0b01101100, 0b00111000, 0b01111100, 0b11100110, 0b10111010, 0b00010000, 0b11000110, 0b00111000,
    0b11111110, 0b10101010, 0b10111010, 0b11000110, 0b10001000, 0b00111000, 0b01111100, 0b01111100, 0b11101110, 0b01111100, 0b11111110, 0b11000110, 0b11111110, 0b00010000, 0b11010110, 0b01111100,
    0b11111110, 0b01010100, 0b10111010, 0b11101110, 0b00100010, 0b01111000, 0b01111100, 0b00111000, 0b01101100, 0b00111000, 0b01010100, 0b11100110, 0b10000010, 0b01110000, 0b11000110, 0b00111000,
    0b11111110, 0b10101010, 0b01111100, 0b01111100, 0b10001000, 0b00001000, 0b00111000, 0b00010000, 0b00111000, 0b00101000, 0b01011100, 0b01111100, 0b11111110, 0b01110000, 0b01111100, 0b00010000,

    0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
    0b00000000, 0b01111100, 0b00010000, 0b01111100, 0b01111100, 0b00000000, 0b00000000, 0b01111100, 0b11111110, 0b10101010, 0b01111100, 0b01111100, 0b01110000, 0b01110000, 0b00000000, 0b00000000,
    0b00000000, 0b11001110, 0b00111000, 0b00111000, 0b11101110, 0b10100000, 0b10001000, 0b11010110, 0b00000000, 0b10101010, 0b11000110, 0b11111110, 0b11011000, 0b11111000, 0b00000000, 0b00000000,
    0b10101010, 0b11000110, 0b11111110, 0b00010000, 0b11000110, 0b01001010, 0b01010100, 0b11101110, 0b11111110, 0b10101010, 0b11000110, 0b11111110, 0b10001000, 0b10001000, 0b00000000, 0b00000000,
    0b00000000, 0b11001110, 0b01111100, 0b00111000, 0b11000110, 0b00000100, 0b00100010, 0b11010110, 0b00000000, 0b10101010, 0b11010110, 0b11101110, 0b11011000, 0b11111000, 0b00000000, 0b00000000,
    0b00000000, 0b01111100, 0b01000100, 0b01111100, 0b01111100, 0b00000000, 0b00000000, 0b01111100, 0b11111110, 0b10101010, 0b01111100, 0b01111100, 0b01110000, 0b01110000, 0b00000000, 0b00000000,
];

pub fn write_font() {
    let mut file = File::create("src/assets/font.raw").unwrap();
    file.write_all(DATA).unwrap();
}
*/
