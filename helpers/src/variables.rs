use crate::err_handling::LogoError;
use crate::maths::math_calculation;
use crate::token_check::{is_bool, is_number};
use crate::{token_check::Prefix, turtle::Turtle};
use colored::Colorize;
use std::collections::HashMap;
use unsvg::Color;
use unsvg::COLORS;

// use a const f32 to store true and false value
// so that variable can only have f32 value
pub const TRUE: f32 = 1.23456;
pub const FALSE: f32 = 6.54321;

pub fn make(variables: &mut HashMap<String, f32>, variable: &str, value: f32) {
    variables.insert(variable.to_string(), value);
}

pub fn addassign(
    variables: &mut HashMap<String, f32>,
    variable: &str,
    value: f32,
) -> Result<(), LogoError> {
    let pre = variables.get(variable);
    match pre {
        Some(pre) => {
            variables.insert(variable.to_string(), value + pre).unwrap();
            Ok(())
        }
        None => Err(LogoError::new(format!(
            "Trying to {} a non-existing variable!",
            "ADDASSIGN".blue()
        ))),
    }
}

pub fn get_number(
    prefix: &Prefix,
    rest: &str,
    turtle: &Turtle,
    variables: &HashMap<String, f32>,
    next_line: &usize,
    tokens: &mut std::str::SplitWhitespace,
) -> Result<f32, LogoError> {
    //logo code start from line 1, while index start from 0, so next line is actually the current line in logo code
    if is_number(prefix) {
        match prefix {
            Prefix::XCOR => Ok(turtle.x),
            Prefix::YCOR => Ok(turtle.y),
            Prefix::HEADING => Ok(turtle.direction as f32),
            Prefix::COLOR => Ok(get_color(turtle.color)),
            Prefix::QuotationValue => Ok(rest.parse::<f32>().unwrap()),
            Prefix::OperatorValue => math_calculation(tokens, rest, next_line, turtle, variables),
            Prefix::Colon => match variables.get(rest) {
                Some(value) => {
                    if value == &TRUE || value == &FALSE {
                        return Err(LogoError::new(format!(
                            "in line {}, variable: {}, is a bool but requires a number!",
                            next_line.to_string().yellow(),
                            rest.red()
                        )));
                    }
                    Ok(*value)
                }
                None => Err(LogoError::new(format!(
                    "in line {}, trying to retrieve a non-existing variable: {}!",
                    next_line.to_string().yellow(),
                    rest.red()
                ))),
            },
            //won't happen here
            _ => Err(LogoError::new("won't happen here".to_string())),
        }
    } else {
        Err(LogoError::new(format!(
            "In line {}, trying to get a number but receving a non-number: {}!",
            next_line.to_string().yellow(),
            rest.red()
        )))
    }
}

pub fn get_bool(prefix: &Prefix, next_line: &usize) -> Result<f32, LogoError> {
    if is_bool(prefix) {
        if prefix == &Prefix::TRUE {
            Ok(TRUE)
        } else if prefix == &Prefix::FALSE {
            Ok(FALSE)
        } else {
            Err(LogoError::new(format!(
                "Error occured in {} in line: {}",
                "get_bool".blue(),
                next_line.to_string().yellow()
            )))
        }
    } else {
        Err(LogoError::new(format!(
            "Trying to get a bool in line {}, but getting a non-bool",
            next_line.to_string().yellow()
        )))
    }
}

pub fn get_color(color: &Color) -> f32 {
    for (i, array_color) in COLORS.iter().enumerate() {
        if color == array_color {
            return i as f32;
        }
    }

    eprintln!("error in get_color!, color: {color:?}");
    0.0
}

pub fn get_int(float: f32, next_line: &usize) -> Result<i32, LogoError> {
    if float.round() != float {
        return Err(LogoError::new(format!(
            "in line: {}, a parameter requires a integer with: {}, but received a non-integer!",
            next_line.to_string().yellow(),
            float.to_string().red()
        )));
    }
    Ok(float as i32)
}

pub fn get_number_or_bool(
    prefix: &Prefix,
    rest: &str,
    turtle: &Turtle,
    variables: &HashMap<String, f32>,
    next_line: &usize,
    tokens: &mut std::str::SplitWhitespace,
) -> Result<f32, LogoError> {
    // check if it is a variable containing a bool
    if prefix == &Prefix::Colon {
        if let Some(value) = variables.get(rest) {
            if value == &TRUE {
                return Ok(TRUE);
            } else if value == &FALSE {
                return Ok(FALSE);
            }
        } else {
            return Err(LogoError::new(format!(
                "in line {}, failed to retrieve a variable: {}",
                next_line.to_string().yellow(),
                rest.red()
            )));
        }
    }

    // check if it is a bool value
    if is_bool(prefix) {
        return get_bool(prefix, next_line);
    }

    // check if it is a operator returning bool
    if prefix == &Prefix::OperatorBool {
        let value = math_calculation(tokens, rest, next_line, turtle, variables)?;
        Ok(value)
    }
    // check if it is a number value or variable
    else if is_number(prefix) {
        return get_number(prefix, rest, turtle, variables, next_line, tokens);
    } else {
        return Err(LogoError::new(format!(
            "in line: {}, trying to get a number or bool with: {}, but failed",
            next_line.to_string().yellow(),
            rest.red()
        )));
    }
}
