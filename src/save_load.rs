use crate::main_window;

pub fn save_canvas(main_window: &main_window::MainWindow<'_>, file_path: &String) -> String{

    //get the file path to the save file
    let path_string;
    let path;
    if file_path == ""{ //if a save file hasn't been selected yet
        path = native_dialog::FileDialog::new()
            .set_location("~")
            .add_filter("Text", &["txt"])
            .set_filename(".txt")
            .show_save_single_file()
            .unwrap().unwrap_or(std::path::PathBuf::new());
        path_string = path.as_path().to_str().unwrap(); 
        
        if path_string == ""{ //eject if they canceled out of the file picker
            return String::from("");
        }
    }
    else{ //if a save file has been selected
        path_string = file_path;
    }
    
    //create the conents of the save file
    let mut save_string = String::new();

    //the array is saved as: {num_of_times character appears in a row}{character}\t
    for row in &main_window.window_array{
        let mut current_char = row[0];
        let mut num_of_char = 0;
        for character in row{
            if *character == current_char{
                num_of_char += 1;
            }
            else{
                save_string.push_str(&(num_of_char.to_string()));
                save_string.push(current_char);
                save_string.push('\t');
                current_char = *character;
                num_of_char = 1;
            }
        } 
        save_string.push_str(&(num_of_char.to_string()));
        save_string.push(current_char);
        save_string.push('\n');
    }

    save_string.push_str(&("num_of_rows:".to_owned() + &(main_window.num_of_rows).to_string() + ":\n"));
    save_string.push_str(&("num_of_cols:".to_owned() + &(main_window.num_of_cols).to_string() + ":\n"));

    //write to save file
    let _ = std::fs::write(&path_string, &save_string).unwrap();

    return String::from(path_string);
}

pub fn load_canvas(main_window: &mut main_window::MainWindow<'_>) -> String{
    
    let path = native_dialog::FileDialog::new()
        .set_location("~")
        .add_filter("Text", &["txt"])
        .show_open_single_file()
        .unwrap().unwrap_or(std::path::PathBuf::new());
    let path_string = path.as_path().to_str().unwrap(); 
    
    if path_string == ""{ //eject if they canceled out of the file picker
        return String::from("");
    }
    
    let file_string = std::fs::read_to_string(path_string).unwrap();
    let mut split_file_string: Vec<&str> = file_string.split("\n").collect();

    //pop the empty line at the end
    let _ = split_file_string.pop().unwrap();
    
    let mut temp_line = split_file_string.pop().unwrap();
    let mut temp_split: Vec<&str> = temp_line.split(":").collect();
    main_window.col_count_change(temp_split[1].parse::<i32>().unwrap());   

    temp_line = split_file_string.pop().unwrap();
    temp_split = temp_line.split(":").collect();
    main_window.row_count_change(temp_split[1].parse::<i32>().unwrap());   
    
    let mut row_count = 0;
    let mut col_count = 0;
    for line in split_file_string{
        let line_split: Vec<&str> = line.split("\t").collect();   
        for entry in line_split{
            let (num, character) = entry.split_at(entry.len() - 1);
            let int_num = num.parse::<i32>().unwrap();
            let char_character: Vec<char> = character.chars().collect();
            for _ in 0..int_num{
                main_window.window_array[row_count as usize][col_count as usize] = char_character[0];
                col_count += 1;
            }
        }
        col_count = 0;
        row_count += 1;
    }

    
    return String::from(path_string);
}
