use tile::tiledata::TileData;
use tile::tileset::TileSet;

mod tile;

fn main() {
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
    let file_path = "/workspaces/gbt/snake.2bpp";
    let result = ts.read_file(file_path);
    
    match result {
        Ok(()) => println!("File loaded"),
        Err(error) => panic!("File not found {error}")
    }
    
    let result = ts.write_file("/workspaces/gbt/snake.test");
    
    match result {
        Ok(()) => println!("File saved"),
        Err(error) => panic!("File not saved {error}")
    }

    let test = &ts.data[1];
    println!("{test}");
}