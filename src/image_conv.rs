
use crate::image::GenericImageView;
use crate::image::Pixel;
use crate::rayon::iter::ParallelIterator;
use crate::image::GenericImage;
use rayon::prelude::*;
use crate::main_window::MainWindow;
use std::fs;
use crate::save_load;

pub fn convert_image_put_in_window(
    main_window: &mut MainWindow<'_>, 
    current_mouse_pos: &[i32; 2], 
    start_mouse_pos: &[i32; 2],
    map_choice: &str,
    draw_lines: bool
){ 

    let path = native_dialog::FileDialog::new()
        .set_location("~")
        .add_filter("Image", &["png", "jpeg"])
        .show_open_single_file()
        .unwrap().unwrap_or(std::path::PathBuf::new());
    let path_string = path.as_path().to_str().unwrap(); 
    
    if path_string == ""{ //eject if they canceled out of the file picker
        return;
    }

    let img = image::open(path_string).unwrap();

    let begr: i32 = start_mouse_pos[0];
    let begc: i32 = start_mouse_pos[1];
    let finr: i32 = current_mouse_pos[0];
    let finc: i32 = current_mouse_pos[1];

    //how many characters is the image being converted to
    let wcount: u32 = ((begc - finc).abs() + 1) as u32;  
    let hcount: u32 = ((begr - finr).abs() + 1) as u32; 

    let mut ascii_output = convert_image(&img, current_mouse_pos, start_mouse_pos, map_choice, draw_lines);

    //do the line operations if requested
    if draw_lines{
        ascii_output = add_lines_to_conv(&img, &ascii_output, wcount, hcount);
    }

    //find top left --> bottom right
    let tl_col = if begc < finc {begc} else {finc};
    let tl_row = if begr < finr {begr} else {finr};
    let br_col = if begc > finc {begc} else {finc};
    let br_row = if begr > finr {begr} else {finr};

    //println!("tl_row {} tl_col {}", tl_row, tl_col);
    //println!("br_row {} br_col {}", br_row, br_col);

    //draw ascii image to window
    let mut conv_col = 0;
    let mut conv_row = 0;
    for row in tl_row..=br_row{
        //println!("---row {} conv_row {}", row, conv_row);
        for col in tl_col..=br_col{
            //println!("col {} conv_col {}", col, conv_col);
            main_window.add_to_preview_buffer(row, col, ascii_output[conv_row as usize][conv_col as usize]);
            conv_col += 1;
        }
        conv_col = 0;
        conv_row += 1;
    }
}

fn convert_image(
    source_img: &image::DynamicImage,
    current_mouse_pos: &[i32; 2], 
    start_mouse_pos: &[i32; 2],
    map_choice: &str,
    draw_lines: bool
) -> Vec<Vec<char>>{
    let begr: i32 = start_mouse_pos[0];
    let begc: i32 = start_mouse_pos[1];
    let finr: i32 = current_mouse_pos[0];
    let finc: i32 = current_mouse_pos[1];
   
    //declare the characters that lumincance values will be mapped to
    let ascii_array: Vec<char>;
    if map_choice == "1"{
        ascii_array = Vec::from([' ', '\u{2581}', '\u{2582}', '\u{2583}', '\u{2584}', '\u{2585}', '\u{2586}', '\u{2587}', '\u{2588}']);
    }
    else if map_choice == "2"{
        //ascii_array = Vec::from([' ', '.', ':', 'c', 'o', 'P', '#', '%', '@', '\u{2586}']);
        ascii_array = Vec::from([' ', '.', ':', 'c', 'o', 'P', 'O', '?', '@', '\u{2586}']);
    }
    else if map_choice == "3" && draw_lines{
        ascii_array = Vec::from([' ']);
    }
    else{
        ascii_array = Vec::from([' ', '.', ':', '-', '=', '+' , '*', '#', '%', '@']);
    }

    let map_length = ascii_array.len() as f32;
    let lum_map_num = 255.0 as f32 / map_length; //the span of luminance each character gets

    let img = source_img.grayscale();
    
    //how many characters is the image being converted to
    let wcount: u32 = ((begc - finc).abs() + 1) as u32;  
    let hcount: u32 = ((begr - finr).abs() + 1) as u32; 

    //downscale the image so a single pixel from the image corresponds to a single character
    let downscaled = img.resize_exact(wcount, hcount, image::imageops::FilterType::CatmullRom);

    let mut ascii_output: Vec<Vec<char>> = Vec::new();
    
    //go through each pixel and map its luminace to its respective character in the ascii array
    for y in 0..hcount{
        ascii_output.push(Vec::new());
        for x in 0..wcount{
            let pixel = downscaled.get_pixel(x, y);
            let lpixel = pixel.to_luma();
               
            let mut ascii_pos: f32 = (lpixel[0] as f32 / lum_map_num).floor() - 1.0;

            if ascii_pos == map_length {ascii_pos -= 1.0;}
            
            ascii_output[y as usize].push(ascii_array[ascii_pos as usize]);
        }
    }
    /*
    for row in &ascii_output{
        for col in row{
            print!("{}", col);
        }
        print!("\n");
    }
    */
    return ascii_output;
}

fn add_lines_to_conv(source_image: &image::DynamicImage, ascii_output: &Vec<Vec<char>>, wcount: u32, hcount: u32) -> Vec<Vec<char>>{
    //goes through the steps of adding edges
    
    let image = source_image.grayscale(); 
    let diff = diff_of_gauss(&image);
    
    let mut sobel_ascii_output = sobel_ascii_filter(&diff);

    sobel_ascii_output = downscale_sobel(&sobel_ascii_output, wcount, hcount);
    
    //write the edge infomation into the ascci image
    let mut row_count = 0;
    let mut col_count = 0;
    for row in ascii_output{
        for col in row{
            if sobel_ascii_output[row_count][col_count] == ' '{
                sobel_ascii_output[row_count][col_count] = *col;
            }
            col_count += 1;
        }
        row_count += 1;
        col_count = 0;
    }
    return sobel_ascii_output;
}


fn diff_of_gauss(image: &image::DynamicImage) -> image::DynamicImage{
    //creates a differance of gaussians from the given image
    //general idea: blur two images at different amounts of blur, subtract them, get edges

    /*
    two approaches to this
    one
        use a small sigma with a small scalar (defaut: sigma = 2 , scalar = 1.6)
        this will create lines will make the DOG lines that trace the edges
        they will be pretty detailed
        when using this, be very strict about your edge threshold when downscaling the sobel filter (ei 0.60)
    two
        use a small sigma with a huge scalar (imo: sigma = 2 , scalar = 60)
        (use .fast_blur for this one)
        will create blobs of white and black, swapping on edges
        use a very lax edge threshold on this one when downscaling the sobel filter (ei 0.20) 
        (the DOG looks cool)
        (can have issue on very simple images ie a simple circle)
    */
    let gaussian_sigma: f32 = 2.0;
    let blurs: Vec<image::DynamicImage> = [gaussian_sigma, gaussian_sigma * 1.6].par_iter().map(|sigma|{
            return image.blur(*sigma);
    }).collect(); 

    let mut diff = image::DynamicImage::new_luma8(image.width(), image.height());
    let buff_diff = diff.clone().into_luma8();

    //subtract the two images to get edges
    let pixel_output: Vec<(u32, u32, image::Rgba<u8>)> = buff_diff.par_enumerate_pixels().map(|pixel_struct|{
        let pixel_one = blurs[0].get_pixel(pixel_struct.0, pixel_struct.1).to_luma();
        let pixel_two = blurs[1].get_pixel(pixel_struct.0, pixel_struct.1).to_luma();
       
        let mut lumi_value;
        
        if pixel_one[0] > pixel_two[0]{
            lumi_value = pixel_one[0] - pixel_two[0];
        } else{
            lumi_value = 0;
        }
        
        //make pixels in the DOG either pure white or pure black
        if lumi_value > 5{
            lumi_value = 255;
        } else{
            lumi_value = 0;
        }
        
        let pixel_new = image::Luma([lumi_value]).to_rgba();

        return (pixel_struct.0, pixel_struct.1, pixel_new);
    }).collect();
    
    //create the diff of gaussians image
    for pixel in &pixel_output{
        diff.put_pixel(pixel.0, pixel.1, pixel.2);
    }
    //diff.save("diff_of_gauss.png").unwrap();

    return diff;
}

fn sobel_ascii_filter(image: &image::DynamicImage) -> Vec<Vec<char>>{
    //performs the sobel operation on the image to get angles of the angles
    //and turn them into corresponging asccii characters

    let full_height = image.height();
    let full_width = image.width();
    /*
    let top_point1 = 30;
    let top_point2 = 60;
    let top_point3 = 120;
    let top_point4 = 150;
    let bot_point1 = top_point1 * -1;
    let bot_point2 = top_point2 * -1;
    let bot_point3 = top_point3 * -1;
    let bot_point4 = top_point4 * -1;
    */

    //bounds mapping angle values to asccii characters
    // top half of circle
    // 180 -- tp4 \ tp3 | tp2 / tp1 -- 0
    let top_point1 = 20;
    let top_point2 = 70;
    let top_point3 = 110;
    let top_point4 = 160;
    // bottom half of circle
    // -179 -- bp4 / bp3 | bp2 \ bp1 -- 0
    let bot_point1 = top_point1 * -1;
    let bot_point2 = top_point2 * -1;
    let bot_point3 = top_point3 * -1;
    let bot_point4 = top_point4 * -1;


    let y_kernel: [[i32 ; 3] ; 3] = [[-1, -2, -1], 
                                     [ 0,  0,  0], 
                                     [ 1,  2,  1]];

    let x_kernel: [[i32 ; 3] ; 3] = [[-1,  0,  1], 
                                     [-2,  0,  2], 
                                     [-1,  0,  1]];

    let buff_image = image.clone().into_luma8();

    //perform the sobel operation
    let pixel_output: Vec<(u32, u32, char)> = buff_image.par_enumerate_pixels().map(|pixel_struct|{
            //deal with edges
            let x = pixel_struct.0;
            let y = pixel_struct.1;
            if x == 0 || x == full_width - 1{
                return (x, y, ' ');   
            }
            else if y == 0 || y == full_height - 1{
                return (x, y, ' ');
            }

            //get all 9 pixels
            let mut nine_pixels = [[0, 0, 0], 
                               [0, 0, 0], 
                               [0, 0, 0]];
            let mut arr_x = 0;
            let mut arr_y = 0;
            for pic_y in y-1..=y+1{
                for pic_x in x-1..=x+1{
                    nine_pixels[arr_y][arr_x] = (buff_image.get_pixel(pic_x, pic_y).to_luma()[0]) as i32;
                    arr_x += 1;
                }
                arr_x = 0;
                arr_y += 1;
            }

            //convolve y
            let mut y_result = 0;
            for temp_y in 0..3{
                for temp_x in 0..3{
                    y_result += y_kernel[temp_y][temp_x] * nine_pixels[temp_y][temp_x]; 
                }
            }
            
            //convolve x
            let mut x_result = 0;
            for temp_y in 0..3{
                for temp_x in 0..3{
                    x_result += x_kernel[temp_y][temp_x] * nine_pixels[temp_y][temp_x]; 
                }
            }
           
            //compute angle of edge (-180, 180] degrees
            let angle = (((y_result as f32).atan2(x_result as f32) * 180.0) / std::f32::consts::PI) as i32;
            

            //translate angle values to ascii characters
            if angle == 0 && x_result < 100{ //if the angle is zero and there wasn't much in the way of a x_gradient
                return (x, y, ' ');
            }
            else if (angle > top_point2 && angle < top_point3) || (angle < bot_point2 && angle > bot_point3){
                return (x, y, '\u{2014}'); //\u{2014}
            }
            else if (angle > top_point4) || (angle < top_point1 && angle >= 0) || (angle < bot_point4) || (angle < 0 && angle > bot_point1){
                return (x, y, '|');
            }
            else if (angle <= top_point3 && angle >= top_point4) || (angle <= bot_point1 && angle >= bot_point2){
                return (x, y, '\\');
            }
            else if (angle >= top_point1 && angle <= top_point2) || (angle <= bot_point3 && angle >= bot_point4) {
                return (x, y, '/'); 
            }
            else {
                return (x, y, ' ');
            }
    }).collect();

    let mut sobel_ascii_output: Vec<Vec<char>> = Vec::new();
      
    //put the ascci angles into an array the dimensions of the original image
    let mut cury = 0;
    sobel_ascii_output.push(Vec::new());
    for x in &pixel_output{
        if x.1 != cury{
            sobel_ascii_output.push(Vec::new());
            cury = x.1;
        }
        sobel_ascii_output[x.1 as usize].push(x.2); 
    }
    /*
    let mut diff = image::DynamicImage::new_luma8(image.width(), image.height());
    for pixel in &pixel_output{
        if pixel.2 != ' '{
            diff.put_pixel(pixel.0, pixel.1, image::Luma([255]).to_rgba());
        }
    }
    diff.save("dsobel.png").unwrap();
    */

    //write_to_file(&sobel_ascii_output, "sobel_outpt.txt");

    return sobel_ascii_output;
}

fn downscale_sobel(sobel_ascii_output: &Vec<Vec<char>>, wcount: u32, hcount: u32) -> Vec<Vec<char>>{
    //downscale the given array of sobel angle characters (from sobel_ascii_filter)

    let full_height = sobel_ascii_output.len() as u32;
    let full_width = sobel_ascii_output[0].len() as u32;
    let y_box_size = (full_height / hcount) as u32;
    let x_box_size = (full_width / wcount) as u32;
    let box_total_pixel = x_box_size * y_box_size as u32;
    let edge_threshold = (box_total_pixel as f32 * 0.05) as u32; //lowering decimal value makes more edges

    //println!("wc {} hc {}", wcount, hcount);
    //println!("x {} y {}", x_box_size, y_box_size);

    //prepare a vector of topleft positions to start each downscale box on
    let mut num_of_rows_added = 0;
    let mut num_of_cols_added = 0;
    let mut boxes: Vec<[u32; 2]>= Vec::new();
    for y in (0..full_height).step_by(y_box_size as usize){
        if num_of_rows_added < hcount{
            for x in (0..full_width).step_by(x_box_size as usize){
                if num_of_cols_added < wcount{
                    boxes.push([x as u32, y as u32]);
                    num_of_cols_added += 1;
                }
            }
        }
        num_of_cols_added = 0;
        num_of_rows_added += 1;
    }

    //boxes.iter().for_each(|cord| println!("{:?}", cord));
   
    //compress the "pixels" of each downscale box into a a single character
    let downscaled: Vec<(u32, u32, char)> = boxes.par_iter().map(|top_left|{
        //count how many of each edge are present in the box
        let mut edge_types = std::collections::HashMap::from([(' ', 0), ('\u{2014}', 0), ('|', 0), ('/', 0), ('\\', 0)]);
        for y in top_left[1]..(top_left[1] + y_box_size){
            for x in top_left[0]..(top_left[0] + x_box_size){
                let map_value = edge_types.get_mut(&sobel_ascii_output[y as usize][x as usize]).unwrap();
                *map_value += 1;
            }
        }
        
        //calculate where in the downscale image this character goes
        let down_y = (top_left[1] / y_box_size) as u32;
        let down_x = (top_left[0] / x_box_size) as u32;

        //find what edge was most prevelant in the box
        let mut max_count = 0;
        let mut max_char = ' ';
        for (chr, count) in edge_types.iter(){
            if *chr == ' '{
                continue;
            }
            if *count > max_count{
                max_count = *count;
                max_char = *chr;
            }
        }
        
        //make sure enough of the edge was present to count it as one
        if max_count >= edge_threshold{
            return(down_x, down_y, max_char);
        }else{
            return(down_x, down_y, ' ');
        }
    }).collect();
   
    let mut downscaled_output = Vec::new();
    for y in 0..hcount{
        downscaled_output.push(Vec::new());
        for _ in 0..wcount{
            downscaled_output[y as usize].push(' ');
        }
    }

    for x in downscaled{
        downscaled_output[x.1 as usize][x.0 as usize] = x.2;
    }

    //write_to_file(&downscaled_output, "output_downscaled.txt");
    return downscaled_output;
}


pub fn create_video_conversion_file(
    current_mouse_pos: &[i32; 2], 
    start_mouse_pos: &[i32; 2],
    map_choice: &str,
    draw_lines: bool
){

    let path = native_dialog::FileDialog::new()
        .set_location("~")
        .show_open_single_dir()
        .unwrap().unwrap_or(std::path::PathBuf::new());
    let path_string = path.as_path().to_str().unwrap(); 
    
    if path_string == ""{ //eject if they canceled out of the file picker
        return;
    }

    let output: fs::ReadDir = fs::read_dir(&path_string).unwrap();
    let mut image_list: Vec<String> = Vec::new();
    for file in output{
        let f = file.unwrap().path();
        let path = f.as_path();
        let ext = path.extension().unwrap();
        if ext == "png" || ext == "jpeg"{
            image_list.push(String::from(path.to_str().unwrap()));
        }
    }
    image_list.sort();
    println!("{:?}", image_list);

    let mut save_string: String = String::new();
    let wcount: u32 = ((start_mouse_pos[1] - current_mouse_pos[1]).abs() + 1) as u32;  
    let hcount: u32 = ((start_mouse_pos[0] - current_mouse_pos[0]).abs() + 1) as u32; 
    save_string.push_str(&("num_of_rows:".to_owned() + &(hcount).to_string() + ":\n"));
    save_string.push_str(&("num_of_cols:".to_owned() + &(wcount).to_string() + ":\n"));
    save_string.push_str(&("frame_per_sec:".to_owned() + ":\n"));

    for image_path in &image_list{
        let source_img = image::open(image_path).unwrap();
        let ascii_output: Vec<Vec<char>> = convert_image(&source_img, current_mouse_pos, start_mouse_pos, map_choice, draw_lines);
        save_load::write_array_to_save_string(&ascii_output, &mut save_string);
        save_string.push_str("---\n");
    }
    
    let _ = std::fs::write(&(String::from(path_string) + "/video_file.txt"), &save_string).unwrap();
}