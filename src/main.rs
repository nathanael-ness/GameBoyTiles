use slint::Window;
//use tile::tiledata::TileData;
use tile::{tiledata::TileData, tileset::TileSet};

mod tile;

slint::slint! {
    import { Button, GridBox } from "std-widgets.slint";
    component PalletColor  {
        in property <int> p_color;
        min-height: 32px;
        min-width: 32px;
        max-height: self.width;        
        Rectangle {
            background: p_color == 0 ? #ffffff :
                        p_color == 1 ? #aaaaaa :
                        p_color == 2 ? #555555 :
                        #000000;
            border-width: p_color == Palette.primary ? 2px : 0px;
            border-color: p_color == Palette.primary ? #ff23ae : #000000;
        }
        TouchArea {
            clicked => {
                Palette.primary = p_color;
            }
        }
    }

    global Palette  {
        in-out property<int> primary: 0;
    }

    component Pixel  {
        in-out property <int> px_color;
        in property <int> x_position;
        in property <int> y_position;
        min-height: 64px;
        min-width: 64px;
        max-height: self.width;
        Rectangle {
            background: px_color == 0 ? #ffffff :
                        px_color == 1 ? #aaaaaa :
                        px_color == 2 ? #555555 :
                        #000000;
            border-width: ta.has-hover ? 2px : 0;
            border-color: #ff23ae;
            ta := TouchArea {}
        }
        TouchArea {
            clicked => {
                root.px_color = Palette.primary;
            }
        }
    }

    export component MainWindow inherits Window {
        in-out property<[[int]]> pixelGrid: [[0,1,2,3,0,1,2,3],[3,2,1,0,3,2,1,0],[0,0,1,1,2,2,3,3],[3,3,2,2,1,1,0,0],[0,1,2,3,0,1,2,3],[3,2,1,0,3,2,1,0],[0,0,1,1,2,2,3,3],[3,3,2,2,1,1,0,0]];
        public function setPixel(x: int, y: int, color: int) {
            pixelGrid[x][y] = color;
        }
        public pure function getPixel(x: int, y: int) -> int { return pixelGrid[x][y]; }
        

        callback load;
        callback save;
        
        min-height: 600px;
        min-width: 800px;
        HorizontalLayout {
            alignment: LayoutAlignment.start;
            spacing: 20px;
            padding: 20px;
            VerticalLayout {
                padding: 15px;
                PalletColor {p_color: 0;}
                PalletColor {p_color: 1;}
                PalletColor {p_color: 2;}
                PalletColor {p_color: 3;}
            }            
            VerticalLayout {
                for row[x] in pixelGrid: HorizontalLayout {
                    for pxColor[y] in row : Pixel {
                        px_color: pixelGrid[x][y];
                        x_position: x;
                        y_position: y;
                    }
                }
            }
            VerticalLayout {
                Button { text: "Load"; clicked => { root.load(); } max-height: 20px;}
                Button { text: "Save"; clicked => { root.save(); } max-height: 20px;}
            }
        }
    }
}

fn main() {

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
        Err(error) => panic!("File not found {error}"),
    }

    let result = ts.write_file("snake.test");

    match result {
        Ok(()) => println!("File saved"),
        Err(error) => panic!("File not saved {error}"),
    }

    
    let window = MainWindow::new().unwrap();

    let main_window_weak = window.as_weak();
    window.on_load(move || {
        let window = main_window_weak.unwrap();
        let image = &ts.data[0];
        for x in 0..8 {
            for y in 0..8 {
                window.invoke_setPixel(x,y,image.get_pixel(x as usize, y as usize) as i32);
            }
        }
    });

    window.on_save(move || {
        let window = main_window_weak.unwrap();
        let mut image: TileData = Default::default();
        for x in 0..8 {
            for y in 0..8 {
                image.set_pixel(x, y, window.invoke_getPixel(x as i32,y as i32));
            }
        }
        ts.data.push(image);
        let _ = ts.write_png();
    });

   
    window.run().unwrap();
    //let _ = ts.write_bmp();
    //let _ = ts.write_png();

    // let test = &ts.data[1];
    // println!("{test}");
}
