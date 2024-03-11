use std::io::{self, Read, Write};
use std::fs::File;
use std::io::BufReader;
use std::fs::OpenOptions;

use super::tiledata::TileData;


#[derive(Debug)]
pub struct TileSet {
    pub data: Vec<TileData>,
    dimentions: [(i32,i32); 128]
}

impl TileSet {
    fn new() -> Self {
        TileSet {
            data: Vec::new(),
            dimentions: [
                // 1      2      3      4      5      6      7      8      9     10     11     12     13     14     15     16
                (1,1), (1,2), (1,3), (2,2), (3,2), (3,2), (2,4), (2,4), (3,3), (2,5), (4,3), (4,3), (3,5), (3,5), (3,5), (4,4),
                // 17     18    19     20     21     22     23     24     25     26     27     28     29     30     31     32
                (3,6), (3,6), (4,5), (4,5), (3,7), (4,6), (4,6), (4,6), (5,5), (4,7), (4,7), (4,7), (5,6), (5,6), (4,8), (4,8),
                // 33     34    35     36     37     38     39     40     41     42     43     44     45     46     47     48
                (6,6), (6,6), (6,6), (6,6), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0),
                // 49     50    51     52     53     54     55     56     57     58     59     60     61     62     63     64
                (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0),
                // 65     66    67     68     69     70     71     72     73     74     75     76     77     78     79     80
                (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0),
                // 81     82    83     84     85     86     87     88     89     90     91     92     93     94     95     96
                (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0),
                // 97     98    99    100    101    102    103    104    105    106    107    108    109    110    111    112
                (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0),
                //113    114   115    116    117    118    119    120    121    122    123    124    125    126    127    128
                (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0)
            ]
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
}