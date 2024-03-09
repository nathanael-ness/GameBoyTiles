use core::fmt;

pub struct TileData {
    pub low: [u8; 8],
    pub high: [u8; 8]
}

impl TileData {
    pub fn set_pixel(&mut self, x: usize, y: usize, color: i32) {
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

    pub fn get_pixel(self, x: usize, y: usize) -> i32 {
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