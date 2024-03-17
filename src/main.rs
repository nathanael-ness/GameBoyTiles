//use tile::tiledata::TileData;
use tile::tileset::TileSet;

mod tile;


slint::slint!{
    component MemoryTile inherits Rectangle {
        width: 64px;
        height: 64px;
        background: #3960D5;
    
        Image {
            source: @image-url("temp/image.png");
            width: parent.width;
            height: parent.height;
        }
    }

    export component MainWindow inherits Window {
        MemoryTile {}
    }
}


fn main() {
    MainWindow::new().unwrap().run().unwrap();

    // let mut td = TileData {
    //     low: [0,0,0,0,0,0,0,0],
    //     high: [0,0,0,0,0,0,0,0]
    // };
    // td.set_pixel(2, 2, 3);
    // td.set_pixel(3, 2, 3);
    // td.set_pixel(4, 2, 3);
    // let px = td.get_pixel(2,2);    
    // println!("{px}");

    //"/workspaces/GBT/snake.2bpp"
    let mut ts: TileSet = TileSet::new();
    let file_path = "snake.2bpp";
    let result = ts.read_file(file_path);
    
    match result {
        Ok(()) => println!("File loaded"),
        Err(error) => panic!("File not found {error}")
    }
    
    let result = ts.write_file("snake.test");
    
    match result {
        Ok(()) => println!("File saved"),
        Err(error) => panic!("File not saved {error}")
    }

    //let _ = ts.write_bmp();
    //let _ = ts.write_png();

    // let test = &ts.data[1];
    // println!("{test}");
}