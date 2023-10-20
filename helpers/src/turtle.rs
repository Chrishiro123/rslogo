use crate::err_handling::LogoError;
use colored::Colorize;
use unsvg::get_end_coordinates;
use unsvg::Color;
use unsvg::Image;
use unsvg::COLORS;

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
            x: (width / 2) as f32,
            y: (height / 2) as f32,
            down: false,
            direction: 0,
            color: &COLORS[7],
        }
    }

    pub fn penup(&mut self) -> Result<(), LogoError> {
        self.down = false;
        Ok(())
    }

    pub fn pendown(&mut self) -> Result<(), LogoError> {
        self.down = true;
        Ok(())
    }

    pub fn setx(&mut self, x: f32) -> Result<(), LogoError> {
        self.x = x;
        Ok(())
    }

    pub fn sety(&mut self, y: f32) -> Result<(), LogoError> {
        self.y = y;
        Ok(())
    }

    pub fn setpencolor(&mut self, color_code: i32) -> Result<(), LogoError> {
        if !(1..=15).contains(&color_code) {
            return Err(LogoError::new(format!(
                "{} need integer between {}!",
                "setpencolor".blue(),
                "[1, 15]".blue()
            )));
        }
        //return reference of the element to prevent copy
        self.color = &COLORS[color_code as usize];
        Ok(())
    }

    pub fn turn(&mut self, degree: i32) -> Result<(), LogoError> {
        self.direction += degree;
        Ok(())
    }

    pub fn setheading(&mut self, degree: i32) -> Result<(), LogoError> {
        self.direction = degree;
        Ok(())
    }

    pub fn forward(
        &mut self,
        numpixels: f32,
        image: &mut Image,
        next_line: &usize,
    ) -> Result<(), LogoError> {
        if self.down {
            match image.draw_simple_line(self.x, self.y, self.direction, numpixels, *self.color) {
                Ok((x, y)) => {
                    self.x = x;
                    self.y = y;
                }
                Err(e) => {
                    return Err(LogoError::new(format!(
                        "in line {}, error occured in {} function: {e}",
                        next_line.to_string().yellow(),
                        "forward".yellow()
                    )));
                }
            }
        } else {
            let (x, y) = get_end_coordinates(self.x, self.y, self.direction, numpixels);
            self.x = x;
            self.y = y;
        }
        Ok(())
    }

    pub fn back(
        &mut self,
        numpixels: f32,
        image: &mut Image,
        next_line: &usize,
    ) -> Result<(), LogoError> {
        if self.down {
            match image.draw_simple_line(
                self.x,
                self.y,
                self.direction + 180,
                numpixels,
                *self.color,
            ) {
                Ok((x, y)) => {
                    self.x = x;
                    self.y = y;
                }
                Err(e) => {
                    return Err(LogoError::new(format!(
                        "in line {}, error occured in {} function: {e:?}",
                        next_line.to_string().yellow(),
                        "BACK".blue()
                    )));
                }
            }
        } else {
            let (x, y) = get_end_coordinates(self.x, self.y, self.direction + 180, numpixels);
            self.x = x;
            self.y = y;
        }
        Ok(())
    }

    pub fn left(
        &mut self,
        numpixels: f32,
        image: &mut Image,
        next_line: &usize,
    ) -> Result<(), LogoError> {
        if self.down {
            match image.draw_simple_line(
                self.x,
                self.y,
                self.direction + 270,
                numpixels,
                *self.color,
            ) {
                Ok((x, y)) => {
                    self.x = x;
                    self.y = y;
                }
                Err(e) => {
                    return Err(LogoError::new(format!(
                        "in line {}, error occured in {} function: {e:?}",
                        next_line.to_string().yellow(),
                        "left".blue()
                    )));
                }
            }
        } else {
            let (x, y) = get_end_coordinates(self.x, self.y, self.direction + 270, numpixels);
            self.x = x;
            self.y = y;
        }
        Ok(())
    }

    pub fn right(
        &mut self,
        numpixels: f32,
        image: &mut Image,
        next_line: &usize,
    ) -> Result<(), LogoError> {
        if self.down {
            match image.draw_simple_line(
                self.x,
                self.y,
                self.direction + 90,
                numpixels,
                *self.color,
            ) {
                Ok((x, y)) => {
                    self.x = x;
                    self.y = y;
                }
                Err(e) => {
                    eprintln!();
                    return Err(LogoError::new(format!(
                        "in line {}, error occured in {} function: {e:?}",
                        next_line.to_string().yellow(),
                        "right".blue()
                    )));
                }
            }
        } else {
            let (x, y) = get_end_coordinates(self.x, self.y, self.direction + 90, numpixels);
            self.x = x;
            self.y = y;
        }
        Ok(())
    }
}
