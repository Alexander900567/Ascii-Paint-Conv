
use crate::image::GenericImageView;
use crate::image::Pixel;

pub fn convert_image_put_in_window(window_array: &mut Vec<Vec<char>>, 
                                   current_mouse_pos: &[i32; 2], 
                                   start_mouse_pos: &[i32; 2]){ 
    let begr: i32 = start_mouse_pos[0];
    let begc: i32 = start_mouse_pos[1];
    let finr: i32 = current_mouse_pos[0];
    let finc: i32 = current_mouse_pos[1];
    
    //let ascii_array = Vec::from([' ', '.', '\'', '`', '^', '"', ',', ':' , ';', 'I', 'l', '!', 'i', '>', '<', '~', '+', '_', '-', '?', ']', '[', '}', '{', '1', ')', '(', '|', '\\', '/', 't', 'f', 'j', 'r', 'x', 'n', 'u', 'v', 'c', 'z', 'X', 'Y', 'U', 'J', 'C', 'L', 'Q', '0', 'O', 'Z', 'm', 'w', 'q', 'p', 'd', 'b', 'k', 'h', 'a', 'o', '*', '#', 'M', 'W', '&', '8', '%', 'B', '@', '$']);
    
    //let ascii_array = Vec::from([' ', '.', ':', '-', '=', '+' , '*', '#', '%', '@']);
    //let ascii_array = Vec::from([' ', '.', ':', 'c', 'o', 'P', 'O', '?', '@', '\u{2586}']);
    let ascii_array = Vec::from([' ', '\u{2581}', '\u{2582}', '\u{2583}', '\u{2584}', '\u{2585}', '\u{2586}', '\u{2587}', '\u{2588}']);

    let map_length = ascii_array.len() as f32;
    let lum_map_num = 255.0 as f32 / map_length;


    //println!("{:?}", ascii_array);

    let mut img = image::open("input.png").unwrap();
    img = img.grayscale();

    let wcount: u32 = ((begc - finc).abs() + 1) as u32; 
    let hcount: u32 = ((begr - finr).abs() + 1) as u32;

    let wscale = img.width() / wcount;
    let hscale = img.height() / hcount;

    let nwid = img.width() / wscale;
    let nhei = img.height() / hscale;

    let downscaled = img.resize_exact(nwid, nhei, image::imageops::FilterType::CatmullRom);

    let mut ascii_output: Vec<Vec<char>> = Vec::new();
      
    for y in 0..nhei{
        ascii_output.push(Vec::new());
        for x in 0..nwid{
            let pixel = downscaled.get_pixel(x, y);
            let lpixel = pixel.to_luma();
               
            let mut ascii_pos: f32 = (lpixel[0] as f32 / lum_map_num).floor() - 1.0;

            if ascii_pos == map_length {ascii_pos -= 1.0;}
            
            ascii_output[y as usize].push(ascii_array[ascii_pos as usize]);
        }
    }

    for row in &ascii_output{
        for col in row{
            print!("{}", col);
        }
        print!("\n");
    }

    //find top left --> bottom right
    let tl_col = if begc < finc {begc} else {finc};
    let tl_row = if begr < finr {begr} else {finr};
    let br_col = if begc > finc {begc} else {finc};
    let br_row = if begr > finr {begr} else {finr};

    //println!("tl_row {} tl_col {}", tl_row, tl_col);
    //println!("br_row {} br_col {}", br_row, br_col);

    let mut conv_col = 0;
    let mut conv_row = 0;
    for row in tl_row..=br_row{
        //println!("---row {} conv_row {}", row, conv_row);
        for col in tl_col..=br_col{
            //println!("col {} conv_col {}", col, conv_col);
            window_array[row as usize][col as usize] = ascii_output[conv_row as usize][conv_col as usize];
            conv_col += 1;
        }
        conv_col = 0;
        conv_row += 1;
    }
}
