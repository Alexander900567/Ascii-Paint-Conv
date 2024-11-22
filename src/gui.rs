use std::path::PathBuf;
use crate::main_window;
use crate::tools;

pub struct Gui{
    pub gui_grid: Vec<Vec<i32>>,
    pub buttons: std::collections::HashMap<i32, Button>,
    pub toggle_groups: std::collections::HashMap<i32, i32>,
    pub num_rows: i32,
    pub num_cols: i32,
    pub row_size: f32,
    pub col_size: f32,
}

impl Gui{
    pub fn new(
        gui_height: u32,
        window_width: u32,
        num_rows: i32,
        num_cols: i32
        ) -> Gui{

        let mut start_grid = Vec::new();
        for _ in 0..num_rows{
            let mut temp = Vec::new();
            for _ in 0..num_cols{
                temp.push(-1);
            }
            start_grid.push(temp);
        }
        /*
        making a button
        (button_id, Button::new(&mut start_grid, &mut start_groups,
                                button_id, top_left, bottom_right,
                                asset_path, is_pressed, toggle_group, visible)

        button_id: i32 = number that represents the button, do not use -1 or the same value for more than one button
        top_left: (i32, i32) = the grid position of the top left of the button
        bottom_right: (i32, i32) = the grid position of the bottom right of the button
        asset_path: &str = the path to the pic
        is_pressed: i32 = does the button start toggled on(1) or off(0), -1 makes the button a oneshot (no toggle state)
        toggle_group: i32 = only one button from a toggle group can be on at once, -1 makes the button independent of all others 
        visible: bool = does the button start visible on the bar
        */
        let mut start_buttons = std::collections::HashMap::new();
        let mut start_groups = std::collections::HashMap::new();
        start_buttons.insert(0, Button::new(&mut start_grid, &mut start_groups,
                                            0, (1, 1), (3, 1),
                                            "Assets/PNGs/free_icon.png", 1, 0, true));
        
        start_buttons.insert(1, Button::new(&mut start_grid, &mut start_groups,
                                            1, (1, 12), (3, 12),
                                            "Assets/PNGs/fill_icon.png", 0, -1, false));

        start_buttons.insert(2, Button::new(&mut start_grid, &mut start_groups,
                                            2, (5, 1), (7, 1),
                                            "Assets/PNGs/line_icon.png", 0, 0, true));

        start_buttons.insert(3, Button::new(&mut start_grid, &mut start_groups,
                                            3, (5, 4), (7, 4), 
                                            "Assets/PNGs/rectangle_icon.png", 0, 0, true));

        start_buttons.insert(4, Button::new(&mut start_grid, &mut start_groups,
                                            4, (1, 4), (3, 4),  //We should never have to use the asset. TODO: Remove button
                                            "Assets/PNGs/1x1_button_disabled.png", 0, 0, true));

        start_buttons.insert(5, Button::new(&mut start_grid, &mut start_groups, 
                                            5, (1, 7), (3, 7),
                                            "Assets/PNGs/text_icon.png", 0, 0, true));

        start_buttons.insert(6, Button::new(&mut start_grid, &mut start_groups, 
                                            6, (5, 7), (7, 7),
                                            "Assets/PNGs/picture_icon.png", 0, 0, true));

        start_buttons.insert(7, Button::new(&mut start_grid, &mut start_groups, 
                                            7, (1, 17), (3, 17),
                                            "Assets/PNGs/undo_icon.png", -1, 0, true));

        start_buttons.insert(8, Button::new(&mut start_grid, &mut start_groups, 
                                            8, (1, 19), (3, 19),
                                            "Assets/PNGs/redo_icon.png", -1, 0, true));

        start_buttons.insert(9, Button::new(&mut start_grid, &mut start_groups, 
                                            9, (5, 19), (7, 19), //TODO: make into clipboard art
                                            "Assets/PNGs/clipboard_icon.png", -1, 0, true));

        start_buttons.insert(10, Button::new(&mut start_grid, &mut start_groups, 
                                            10, (1, 12), (3, 12),
                                            "Assets/PNGs/mode_1_icon.png", 0, 1, false));

        start_buttons.insert(11, Button::new(&mut start_grid, &mut start_groups, 
                                            11, (1, 13), (3, 13),
                                            "Assets/PNGs/mode_2_icon.png", 0, 1, false));

        start_buttons.insert(12, Button::new(&mut start_grid, &mut start_groups, 
                                            12, (1, 14), (3, 14),
                                            "Assets/PNGs/mode_3_icon.png", 0, 1, false));

        start_buttons.insert(13, Button::new(&mut start_grid, &mut start_groups, 
                                            13, (1, 15), (3, 15),
                                            "Assets/PNGs/mode_4_icon.png", 1, 1, false));

        start_buttons.insert(14, Button::new(&mut start_grid, &mut start_groups, 
                                            14, (5, 12), (7, 13),
                                            "Assets/PNGs/mode_edge_icon.png", 0, -1, false));

        start_buttons.insert(15, Button::new(&mut start_grid, &mut start_groups,
                                            15, (1, 10), (3, 10),
                                            "Assets/PNGs/select_icon.png", 0, 0, true));

        start_buttons.insert(16, Button::new(&mut start_grid, &mut start_groups,
                                            16, (1, 12), (3, 13),
                                            "Assets/PNGs/clear_icon.png", -1, 0, false));

        start_buttons.insert(17, Button::new(&mut start_grid, &mut start_groups,
                                            17, (5, 12), (7, 13),
                                            "Assets/PNGs/ellipse_icon.png", 0, -1, false));

        Gui {
        gui_grid: start_grid,
            buttons: start_buttons,
            toggle_groups: start_groups,
            num_rows: num_rows,
            num_cols: num_cols,
            row_size: (gui_height as i32 / num_rows) as f32,
            col_size: (window_width as i32 / num_cols) as f32,

        }
    }

    pub fn handle_gui_click(&mut self, x: i32, y: i32, main_window: &mut main_window::MainWindow<'_>, toolbox: &mut tools::Toolbox){
        let grid_pos = self.get_gui_grid_pos(x, y);
        
        let clicked_id = self.gui_grid[grid_pos.0 as usize][grid_pos.1 as usize];

        self.handle_click(clicked_id, main_window, toolbox);
    }

    pub fn handle_click(&mut self, clicked_id: i32, main_window: &mut main_window::MainWindow<'_>, toolbox: &mut tools::Toolbox){
        if clicked_id == -1{
            return;
        }
        let clicked_is_pressed = self.buttons.get(&clicked_id).unwrap().is_pressed;
        let toggle_group = self.buttons.get(&clicked_id).unwrap().toggle_group;

        let mut unclick_id = 0;
        if clicked_is_pressed != -1 {
            unclick_id = self.handle_unclick(toggle_group, clicked_id, main_window, toolbox);
        }

        if clicked_id == 0{
            self.click_free(toolbox);
        }
        else if clicked_id == 2{
            self.click_line(toolbox);
        }
        else if clicked_id == 3{
            self.click_rect(toolbox);
        }
        else if clicked_id == 4{
            self.click_circle(toolbox);
        }
        else if clicked_id == 5{
            self.click_text(toolbox);
        }
        else if clicked_id == 6{
            self.click_picture(toolbox);
        }
        else if clicked_id == 7{
            self.click_undo(main_window);
        }
        else if clicked_id == 8{
            self.click_redo(main_window);
        }
        else if clicked_id == 9{
            self.click_copy_to_clipboard(main_window);
        }
        else if Vec::from([10, 11, 12, 13]).contains(&clicked_id){
            self.click_ascii_pallete(toolbox, clicked_id);
        }
        else if clicked_id == 15{
            self.click_rectangle_selector(toolbox);
        }
        else if clicked_id == 16{
            self.click_reset_box(main_window, toolbox);
        }

        if toggle_group == -1 && clicked_is_pressed == 0{
            if clicked_id == 1{
                self.click_fill(toolbox);
            }
            if clicked_id == 14{
                self.click_edge(toolbox);
            }
            if clicked_id == 17{
                self.click_ellipse(toolbox);
            }
        }

        if clicked_is_pressed != -1 && toggle_group != -1{
            self.buttons.get_mut(&unclick_id).unwrap().is_pressed = 0;
            self.buttons.get_mut(&clicked_id).unwrap().is_pressed = 1;
            self.toggle_groups.insert(toggle_group, clicked_id);
        }
        else if toggle_group == -1 && clicked_is_pressed == 0{
            self.buttons.get_mut(&clicked_id).unwrap().is_pressed = 1;
        }
        else if toggle_group == -1 && clicked_is_pressed == 1{
            self.buttons.get_mut(&clicked_id).unwrap().is_pressed = 0;
        }
    }

    pub fn handle_unclick(&mut self, toggle_group: i32, clicked_id: i32, main_window: &mut main_window::MainWindow<'_>, toolbox: &mut tools::Toolbox) -> i32{
        let unclick_id: i32; 
        let pressed_status: i32;
        if toggle_group == -1{
            unclick_id = clicked_id;
            pressed_status = self.buttons.get(&clicked_id).unwrap().is_pressed;
        }
        else{
            unclick_id = *self.toggle_groups.get(&toggle_group).unwrap();
            pressed_status = -1;
        }
        
        if unclick_id == 3{
            self.unclick_rect(); 
        }
        else if unclick_id == 4{
            self.unclick_circle();
        }
        else if unclick_id == 6{
            self.unclick_picture();
        }
        else if unclick_id == 15{
            self.unclick_rectangle_selector(main_window, toolbox);
        }

        if toggle_group == -1 && pressed_status == 1{
            if unclick_id == 1{
                self.unclick_fill(toolbox);
            }
            if unclick_id == 14{
                self.unclick_edge(toolbox);
            }
            if unclick_id == 17{
                self.unclick_ellipse(toolbox);
            }
        }

        return unclick_id;
    }

    pub fn get_gui_grid_pos(&self, x: i32, y: i32) -> (i32, i32){
        let mut row_pos = (y as f32 / self.row_size) as i32;
        let mut col_pos = (x as f32 / self.col_size) as i32;

        if row_pos >= self.num_rows {row_pos = self.num_rows - 1;}
        else if row_pos < 0 {row_pos = 0;}
        if col_pos >= self.num_cols {col_pos = self.num_cols - 1;}
        else if col_pos < 0 {col_pos = 0;}

        return (row_pos, col_pos);
    }

    pub fn hide_button(&mut self, button_id: i32){
        self.buttons.get_mut(&button_id).unwrap().visible = false;
        let grid_pos = &self.buttons.get(&button_id).unwrap().grid_pos;

        for pos in grid_pos{
            self.gui_grid[pos.0 as usize][pos.1 as usize] = -1;
        }
    }

    pub fn show_button(&mut self, button_id: i32){
        self.buttons.get_mut(&button_id).unwrap().visible = true;
        let grid_pos = &self.buttons.get(&button_id).unwrap().grid_pos;

        for pos in grid_pos{
            self.gui_grid[pos.0 as usize][pos.1 as usize] = button_id;
        }
    }

    //------------------button functions
    
    fn click_free(&self, toolbox: &mut tools::Toolbox){
        toolbox.current_tool = String::from("f");
    }

    fn click_line(&self, toolbox: &mut tools::Toolbox){
        toolbox.current_tool = String::from("l");
    }

    fn click_rectangle_selector(&mut self, toolbox: &mut tools::Toolbox){
        toolbox.current_tool = String::from("a");
        self.show_button(16);
    }
    fn unclick_rectangle_selector(&mut self, main_window: &mut main_window::MainWindow<'_>, toolbox: &mut tools::Toolbox){
        toolbox.rect_sel_tool.reset_box(main_window);
        self.hide_button(16);
    }

    fn click_reset_box(&mut self, main_window: &mut main_window::MainWindow<'_>, toolbox: &mut tools::Toolbox){
        toolbox.rect_sel_tool.reset_box(main_window);
    }

    fn click_rect(&mut self, toolbox: &mut tools::Toolbox){
        self.show_button(1);
        toolbox.current_tool = String::from("r");
    }
    fn unclick_rect(&mut self){
        self.hide_button(1);
    }

    fn click_circle(&mut self, toolbox: &mut tools::Toolbox){
        self.show_button(1);
        self.show_button(17);
        toolbox.current_tool = String::from("o");
    }
    fn unclick_circle(&mut self){
        self.hide_button(1);
        self.hide_button(17);
    }

    fn click_fill(&self, toolbox: &mut tools::Toolbox){
        toolbox.filled = true;
    }
    fn unclick_fill(&self, toolbox: &mut tools::Toolbox){
        toolbox.filled = false;
    }

    fn click_ellipse(&self, toolbox: &mut tools::Toolbox){
        toolbox.ellipse = true;
    }
    fn unclick_ellipse(&self, toolbox: &mut tools::Toolbox){
        toolbox.ellipse = false;
    }

    fn click_ascii_pallete(&self, toolbox: &mut tools::Toolbox, clicked_id: i32){
        if clicked_id == 10{
            toolbox.ascii_type = String::from("1");
        }
        else if clicked_id == 11{
            toolbox.ascii_type = String::from("2");
        }
        else if clicked_id == 12{
            toolbox.ascii_type = String::from("3");
        }
        else if clicked_id == 13{
            toolbox.ascii_type = String::from("4");
        }
    }

    fn click_edge(&self, toolbox: &mut tools::Toolbox){
        toolbox.ascii_edges = true;
    }
    fn unclick_edge(&self, toolbox: &mut tools::Toolbox){
        toolbox.ascii_edges = false;
    }

    fn click_text(&self, toolbox: &mut tools::Toolbox){
        toolbox.current_tool = String::from("t");
    }

    fn click_picture(&mut self, toolbox: &mut tools::Toolbox){
        self.show_button(10);
        self.show_button(11);
        self.show_button(12);
        self.show_button(13);
        self.show_button(14);
        toolbox.current_tool = String::from("p");
    }
    fn unclick_picture(&mut self){
        self.hide_button(10);
        self.hide_button(11);
        self.hide_button(12);
        self.hide_button(13);
        self.hide_button(14);
    }

    fn click_undo(&mut self, main_window: &mut main_window::MainWindow<'_>){
        main_window.undo_redo.perform_undo(&mut main_window.window_array);
    }

    fn click_redo(&mut self, main_window: &mut main_window::MainWindow<'_>){
        main_window.undo_redo.perform_redo(&mut main_window.window_array);
    }

    fn click_copy_to_clipboard(&mut self, main_window: &mut main_window::MainWindow<'_>){
        main_window.copy_to_clipboard();
    }
}

pub struct Button{
    pub button_id: i32,
    pub top_left: (i32, i32),
    pub bottom_right: (i32, i32),
    pub grid_pos: Vec<(i32, i32)>,
    pub asset_path: PathBuf,
    pub is_pressed: i32, //0 unpressed, 1 pressed, -1 one-shot button
    pub toggle_group: i32, //-1 toggle
    pub visible: bool,
}

impl Button{
    pub fn new(
        gui_grid: &mut Vec<Vec<i32>>,
        toggle_groups: &mut std::collections::HashMap<i32, i32>,
        button_id: i32,
        top_left: (i32, i32),
        bottom_right: (i32, i32),
        asset_path: &str,
        is_pressed: i32, //0 unpressed, 1 pressed, -1 one-shot button
        toggle_group: i32,
        visible: bool,
        ) -> Button{

        let mut start_pos = Vec::new();
        for row in top_left.0..=bottom_right.0{
            for col in top_left.1..=bottom_right.1{
                start_pos.push((row, col));
                if visible{
                    gui_grid[row as usize][col as usize] = button_id; 
                }
            }
        }

        if is_pressed == 1 && toggle_group != -1{
            toggle_groups.insert(toggle_group, button_id);
        }
        else if toggle_group > -1 && is_pressed != -1{
            if !toggle_groups.contains_key(&toggle_group){
                toggle_groups.insert(toggle_group, -1);
            }
        }

        Button{
            button_id: button_id,
            top_left: top_left,
            bottom_right: bottom_right,
            grid_pos: start_pos,
            asset_path: PathBuf::from(asset_path),
            is_pressed: is_pressed,
            toggle_group: toggle_group,
            visible: visible,
        }
    }
}
 

