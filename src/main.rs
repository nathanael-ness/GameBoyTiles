//use tile::tiledata::TileData;
use tile::tileset::TileSet;

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
        }
    }

    component Pixel  {
        in property <int> px_color;
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
    }

    export component MainWindow inherits Window {
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
                for x in [0,1,2,3,4,5,6,7]: HorizontalLayout {
                    for y in [0,1,2,3,4,5,6,7] : Pixel {
                        px_color: Math.mod((x + y), 3);
                        x_position: x;
                        y_position: y;
                    }
                }
            }
        }
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
        Err(error) => panic!("File not found {error}"),
    }

    let result = ts.write_file("snake.test");

    match result {
        Ok(()) => println!("File saved"),
        Err(error) => panic!("File not saved {error}"),
    }

    //let _ = ts.write_bmp();
    //let _ = ts.write_png();

    // let test = &ts.data[1];
    // println!("{test}");
}
