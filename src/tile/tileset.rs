use std::io::{self, Read, Write};
use std::fs::File;
use std::io::BufReader;
use std::fs::OpenOptions;

use super::tiledata::TileData;


#[derive(Debug)]
pub struct TileSet {
    pub data: Vec<TileData>
}

impl TileSet {
    pub fn new() -> Self {
        TileSet {
            data: Vec::new()
        }
    }
    pub fn read_file(&mut self, file_path: &str) -> io::Result<()> {
        let f = File::open(file_path)?;
        let mut reader = BufReader::new(f);
        let mut buffer: Vec<u8> = Vec::new();
        reader.read_to_end(&mut buffer)?;

        while buffer.len() > 0
        {
            let raw_tile_data = buffer.split_off(16);

            self.data.push(TileData {
                low: [buffer[0],buffer[2],buffer[4],buffer[6],buffer[8],buffer[10],buffer[12],buffer[14]],
                high: [buffer[1],buffer[3],buffer[5],buffer[7],buffer[9],buffer[11],buffer[13],buffer[15]]
            });
            buffer = raw_tile_data;
        }

        Ok(())
    }

    pub fn write_file(&self, filepath: &str) -> io::Result<()> {
        let mut f = OpenOptions::new().write(true).create(true).open(filepath)?;
        let tileset = &self.data;
        for tile in tileset {
            for i in 0..7 {
                f.write(&[tile.low[i]])?;
                f.write(&[tile.high[i]])?;
            }
        }
        Ok(())
    }

    pub fn write_bmp(&self) -> io::Result<()> {
        let len = self.data.len();
        for image_index in 0..len {
            let path = format!("temp/image{image_index}.bmp");
            let mut f = OpenOptions::new().write(true).create(true).open(path)?;
            let file_header_size: i32 = 14;
            let bitmap_header_size: i32 = 40;
            let pallet_size: i32 = 64;
            let width: i32 = 8;
            let height: i32 = 8;
            let width_padding = if width % 8 == 0 { width } else { width + (8 - (width % 8)) };
            let image_data_size = width_padding * height;
            let file_size: i32 = file_header_size + bitmap_header_size + pallet_size + image_data_size;
            // file header
            f.write("BM".as_bytes())?;
            f.write(&file_size.to_le_bytes())?; // file size
            f.write(&0_i32.to_le_bytes())?; // unused
            f.write(&114_u32.to_le_bytes())?; // data offset
            // bitmap header
            f.write(&bitmap_header_size.to_le_bytes())?; // header size
            f.write(&width.to_le_bytes())?; // width
            f.write(&height.to_le_bytes())?; // height
            f.write(&1_u16.to_le_bytes())?; // layers
            f.write(&4_u16.to_le_bytes())?; // bits per pixel
            f.write(&0_i32.to_le_bytes())?; // compression
            f.write(&0_u32.to_le_bytes())?; // image data size
            f.write(&2835_i32.to_le_bytes())?; // width pixels/meter
            f.write(&2835_i32.to_le_bytes())?; // height pixels/meter
            f.write(&16_i32.to_le_bytes())?; // colors used
            f.write(&0_i32.to_le_bytes())?; // important colors
            // pallet data
            f.write(&16777215_u32.to_le_bytes())?; // white
            f.write(&11184810_u32.to_le_bytes())?; // light grey
            f.write(&5592405_u32.to_le_bytes())?; // dark grey
            f.write(&0_u32.to_le_bytes())?; // black
            for _ in 1..12 {
                f.write(&0_u32.to_le_bytes())?; // black
            }
            // image data
            let px = &self.data[image_index];
            for y in (0..8).rev() {
                let mut line: u32 = px.get_pixel(0, y);
                for x in 1..8 {
                    line = line << 4;
                    line += px.get_pixel(x, y);
                }
                f.write(&line.to_be_bytes())?;
            }            
        }
        Ok(())
    }
}