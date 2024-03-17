use std::io::{self, BufWriter, Read, Write};
use std::fs::File;
use std::io::BufReader;
use std::fs::OpenOptions;
use std::path::Path;

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
            let pallet_size: i32 = 16;
            let width: i32 = 8;
            let height: i32 = 8;
            let width_padding = if  ((width * 4) % 32) == 0 { width / 2 } else {(((width * 4) as f32 / 32_f32).floor() as i32 + 1) / 2};
            let image_data_size = width_padding * height;
            let file_size: i32 = file_header_size + bitmap_header_size + pallet_size + image_data_size;
            //146
            // file header
            f.write("BM".as_bytes())?;
            f.write(&file_size.to_le_bytes())?; // file size
            f.write(&0_i32.to_le_bytes())?; // unused
            f.write(&70_u32.to_le_bytes())?; // data offset
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
            f.write(&3_i32.to_le_bytes())?; // colors used
            f.write(&3_i32.to_le_bytes())?; // important colors
            // pallet data
            f.write(&16777215_u32.to_le_bytes())?; // white
            f.write(&11184810_u32.to_le_bytes())?; // light grey
            f.write(&5592405_u32.to_le_bytes())?; // dark grey
            f.write(&0_u32.to_le_bytes())?; // black

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

    pub fn write_png(&self) -> io::Result<()> {
        let path = Path::new("temp/image.png");
        let file = OpenOptions::new().write(true).create(true).open(path).unwrap();
        let ref mut w = BufWriter::new(file);

        let mut encoder: png::Encoder<'_, &mut BufWriter<File>> = png::Encoder::new(w, 8, 8); // Width is 2 pixels and height is 1.
        encoder.set_color(png::ColorType::Grayscale);
        encoder.set_depth(png::BitDepth::Two);
       
        let mut writer = encoder.write_header().unwrap();

        let mut data: [u8; 16] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
        let px = &self.data[0];
            for y in 0..8 {
                let mut line: u8 = 3 - px.get_pixel(0, y) as u8;
                for x in 1..4 {
                    line = line << 2;
                    line += 3 - px.get_pixel(x, y) as u8;
                }
                data[y*2] = line;
                line = 3 - px.get_pixel(4, y) as u8;
                for x in 5..8 {
                    line = line << 2;
                    line += 3 - px.get_pixel(x, y) as u8;
                }
                data[(y*2)+1] = line;
            }    
        //let data = [255, 0, 0, 255, 0, 0, 0, 255]; // An array containing a RGBA sequence. First pixel is red and second pixel is black.
        writer.write_image_data(&data).unwrap(); // Save
        Ok(())
    }
}