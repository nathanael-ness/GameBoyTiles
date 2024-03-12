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

    pub fn write_bmp(&self, filepath: &str) -> io::Result<()> {
        let mut f = OpenOptions::new().write(true).create(true).open(filepath)?;
        f.write("BM".as_bytes());
        f.write(0); // file size
        f.write(0.as_bytes()); // unused
        f.write(0.as_bytes()); // data offset
        f.write(40.as_bytes()); // header size
        f.write(1.as_buffer()); // width
        f.write(1.as_bytes()); // height
        f.write(1_u16.as_bytes()); // layers
        f.write(4_u16.as_bytes()); // bits per pixel
        f.write(0.as_bytes()); // compression
        f.write(0.as_bytes()); // image data size
        f.write(1.as_buffer()); // width pixels/meter
        f.write(1.as_buffer()); // height pixels/meter
        f.write(16.as_buffer()); // colors used
        f.write(0.as_buffer()); // important colors
        f.write(64.as_buffer()); // color table size
        f.write(4294967040.as_bytes()); // white
        f.write(2863311360.as_bytes()); // light grey
        f.write(1431655680.as_bytes()); // dark grey
        f.write(0.as_bytes()); // black
        // image data
        f.write(0.as_bytes());
        OK(())
    }
}