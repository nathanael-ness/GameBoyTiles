use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;

use tiledata::TileData;

pub mod tiledata;

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

    //"/workspaces/GBT/snake.2bpp"
    let mut ts: TileSet = TileSet{ data: Vec::new() };
    let file_path = "/workspaces/GBT/snake.2bpp";
    let result = ts.read_file(file_path);
    
    match result {
        Ok(()) => println!("File loaded"),
        Err(error) => panic!("File not found {error}")
    }
    
    
    let test = &ts.data[1];
    println!("{test}");
    Ok(())
}

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
}