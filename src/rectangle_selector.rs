pub struct RectangleSelector{
    active: bool,
    rect_contents: Vec<(i32, i32, char)>,
    rect_top_left: (i32, i32),
    rect_bot_right: (i32, i32),
}



impl RectangleSelector{
    pub fn new() -> RectangleSelector{
        RectangleSelector{
            active: false,
            rect_contents: Vec::new(),
            rect_top_left: (-1, -1),
            rect_bot_right: (-1, -1),
        }
    }
    
    pub fn on_mouse_down(&mut self, gpos: &[i32; 2]){
        if !self.active{
            self.rect_top_left = (gpos[0], gpos[1]);
            self.rect_bot_right = (gpos[0], gpos[1]);
        }
    }


    pub fn on_mouse_move(&mut self){
        if !self.active{

            
        }
    }

    pub fn on_mouse_up(&mut self){
        if !self.active{
            self.active = true
        }

    }





}
