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
}