use core::fmt;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;

fn main() -> io::Result<()> {
    println!("Test");
    let mut td = TileData {
        low: [0,0,0,0,0,0,0,0],
        high: [0,0,0,0,0,0,0,0]
    };
    td.set_pixel(2, 2, 3);
    td.set_pixel(3, 2, 3);
    td.set_pixel(4, 2, 3);
    let px = td.get_pixel(2,2);    
    println!("{px}");

    let f = File::open("/workspaces/GBT/snake.2bpp")?;
    let mut reader = BufReader::new(f);
    let mut buffer: Vec<u8> = Vec::new();
    reader.read_to_end(&mut buffer)?;
    
    let mut tile_set: Vec<TileData> = Vec::new();
    while buffer.len() > 0
    {
        let raw_tile_data = buffer.split_off(16);

        tile_set.push(TileData {
            low: [buffer[0],buffer[2],buffer[4],buffer[6],buffer[8],buffer[10],buffer[12],buffer[14]],
            high: [buffer[1],buffer[3],buffer[5],buffer[7],buffer[9],buffer[11],buffer[13],buffer[15]]
        });
        buffer = raw_tile_data;
    }
    let test = &tile_set[1];
    println!("{test}");
    Ok(())
}

pub struct TileData {
    low: [u8; 8],
    high: [u8; 8]
}

impl TileData {
    fn set_pixel(&mut self, x: usize, y: usize, color: i32) {
        if color < 0 || color >= 4 {
            return;
        }
        let mut low = self.low[y];
        let mut high = self.high[y];

        let mut low_bit: u8;
        let mut high_bit: u8;
        match color {
            0 => low_bit = 0,
            1 => low_bit = 1,
            2 => low_bit = 0,
            3 => low_bit = 1,
            _ => return
}        match color {
            0 => high_bit = 0,
            1 => high_bit = 0,
            2 => high_bit = 1,
            3 => high_bit = 1,
            _ => return
        }
        low_bit <<= 7 - x;
        high_bit <<= 7 - x;

        low ^= low_bit;
        high ^= high_bit;

        self.low[y] = low;
        self.high[y] = high;
    }

    fn get_pixel(self, x: usize, y: usize) -> i32 {
        let mut low = self.low[y];
        let mut high = self.high[y];
        let mask: u8 = 1 << 7 - x;
        low &= mask;
        high &= mask;
        let h = high >> 6 - x;
        let l = low >> 7 - x;
        return (h + l) as i32;
    }
}

impl fmt::Display for TileData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:08b}\n{:08b}\n\n{:08b}\n{:08b}\n\n{:08b}\n{:08b}\n\n{:08b}\n{:08b}\n\n{:08b}\n{:08b}\n\n{:08b}\n{:08b}\n\n{:08b}\n{:08b}\n\n{:08b}\n{:08b}\n", self.low[0], self.high[0], self.low[1], self.high[1], self.low[2], self.high[2], self.low[3], self.high[3], self.low[4], self.high[4], self.low[5], self.high[5], self.low[6], self.high[6], self.low[7], self.high[7])
    }
}