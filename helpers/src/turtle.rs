use unsvg::Image;
use unsvg::Color;
use unsvg::COLORS;
use unsvg::get_end_coordinates;

pub struct Turtle {
    pub x: f32,
    pub y: f32,
    pub down: bool,
    pub direction: i32,
    // use reference but not copy from unsvg::COLORS
    pub color: &'static Color,
}

impl Turtle {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            x : (width / 2) as f32,
            y : (height / 2) as f32,
            down : false,
            direction: 0,
            color : &COLORS[7],
        }

    }

    pub fn penup(&mut self) -> Result<(), ()> {
        self.down = false;
        return Ok(());
    }

    pub fn pendown(&mut self) -> Result<(), ()> {
        self.down = true;
        return Ok(());
    }

    pub fn setx(&mut self, x: f32) -> Result<(), ()> {
        self.x = x;
        return Ok(());
    }

    pub fn sety(&mut self, y: f32) -> Result<(), ()> {
        self.y = y;
        return Ok(());
    }

    pub fn setpencolor(&mut self, color_code: i32) -> Result<(), ()> {
        if color_code < 1 || color_code > 15 {
            return Err(());
        }
        //return reference of the element to prevent copy
        self.color = &COLORS[color_code as usize];
        return Ok(())
    }

    pub fn turn(&mut self, degree: i32) -> Result<(), ()> {
        self.direction += degree;
        return Ok(());
    }

    pub fn setheading(&mut self, degree: i32) -> Result<(), ()> {
        self.direction = degree;
        return Ok(());
    }

    pub fn forward(&mut self, numpixels: f32, image: &mut Image) -> Result<(), ()> {
        if self.down == true {
            match image.draw_simple_line(self.x, self.y, self.direction, numpixels, *self.color) {
                Ok((x, y)) => {
                    self.x = x;
                    self.y = y;
                }
                Err(e) => {
                    eprintln!("error occured in forward function: {e:?}");
                    return Err(());
                }
            }
        }
        else {
            let (x, y) = get_end_coordinates(self.x, self.y, self.direction, numpixels);
            self.x = x;
            self.y = y;
        }
        return Ok(());
    }

    pub fn back(&mut self, numpixels: f32, image: &mut Image) -> Result<(), ()> {
        if self.down == true {
            match image.draw_simple_line(self.x, self.y, self.direction + 180, numpixels, *self.color) {
                Ok((x, y)) => {
                    self.x = x;
                    self.y = y;
                }
                Err(e) => {
                    eprintln!("error occured in back function: {e:?}");
                    return Err(());
                }
            }
        }
        else {
            let (x, y) = get_end_coordinates(self.x, self.y, self.direction + 180, numpixels);
            self.x = x;
            self.y = y;
        }
        return Ok(());
    }

    pub fn left(&mut self, numpixels: f32, image: &mut Image) -> Result<(), ()> {
        if self.down == true {
            match image.draw_simple_line(self.x, self.y, self.direction + 270, numpixels, *self.color) {
                Ok((x, y)) => {
                    self.x = x;
                    self.y = y;
                }
                Err(e) => {
                    eprintln!("error occured in left function: {e:?}");
                    return Err(());
                }
            }
        }
        else {
            let (x, y) = get_end_coordinates(self.x, self.y, self.direction + 270, numpixels);
            self.x = x;
            self.y = y;
        }
        return Ok(());
    }

    pub fn right(&mut self, numpixels: f32, image: &mut Image) -> Result<(), ()> {
        if self.down == true {
            match image.draw_simple_line(self.x, self.y, self.direction + 90, numpixels, *self.color){
                Ok((x, y)) => {
                    self.x = x;
                    self.y = y;
                }
                Err(e) => {
                    eprintln!("error occured in right function: {e:?}");
                    return Err(());
                }
            }
        }
        else {
            let (x, y) = get_end_coordinates(self.x, self.y, self.direction + 90, numpixels);
            self.x = x;
            self.y = y;
        }
        return Ok(());
    }
}
