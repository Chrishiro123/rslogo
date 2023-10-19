use std::collections::HashMap;
use crate::maths::math_calculation;
use crate::token_check::{is_number, is_bool};
use crate::{token_check::Prefix, turtle::Turtle};
use unsvg::Color;
use unsvg::COLORS;

// use a const f32 to store true and false value
// so that variable can only have f32 value
pub const TRUE: f32 = 1.23456;
pub const FALSE: f32 = 6.54321;

pub fn make(variables: &mut HashMap<String, f32>, variable: &str, value: f32) {
    variables.insert(variable.to_string(), value);
}

pub fn addassign<'a>(variables: &mut HashMap<String, f32>, variable: &str, value: f32) -> Result<(), ()> {
    let pre = variables.get(variable);
    match pre {
        Some(pre) => {
            variables.insert(variable.to_string(), value + pre).unwrap();
            return Ok(());
        },
        None => {
            eprintln!("Trying to ADDASSIGN a non-existing variable!");
            return Err(());
        }
    }
}

pub fn get_number(prefix: &Prefix, 
    rest: &str, 
    turtle : &Turtle, 
    variables: &HashMap<String, f32>, 
    next_line: &usize,
    tokens: &mut std::str::SplitWhitespace,
) -> Result<f32, ()> {
    //logo code start from line 1, while index start from 0, so next line is actually the current line in logo code
    if is_number(prefix) {
        match prefix {
            &Prefix::XCOR => Ok(turtle.x),
            &Prefix::YCOR => Ok(turtle.y),
            &Prefix::HEADING => Ok(turtle.direction as f32),
            &Prefix::COLOR => Ok(get_color(turtle.color)),
            &Prefix::QuotationValue => Ok(rest.parse::<f32>().unwrap()),
            &Prefix::OperatorValue => return math_calculation(tokens, rest, next_line, turtle, variables),
            &Prefix::Colon => {
                match variables.get(rest) {
                    Some(value) => {
                        if value == &TRUE || value == &FALSE {
                            eprintln!("in line {next_line}, variable: {rest}, is a bool but requires a number!");
                            return Err(());
                        }
                        return Ok(*value)
                    },
                    None => {
                        eprintln!("in line {next_line}, trying to retrieve a non-existing variable: {rest}!");
                        return Err(());
                    },
                }
            },
            //won't happen here
            _ => return Err(()),
        }
    }
    else {
        eprintln!("In line {next_line}, trying to get a number but receving a non-number: {rest}!");
        return Err(());
    }
}

pub fn get_bool(prefix: &Prefix, next_line: &usize) -> Result<f32, ()> {
    if is_bool(prefix) {
        if prefix == &Prefix::TRUE {
            return Ok(TRUE);
        }
        else if prefix == &Prefix::FALSE{
            return Ok(FALSE);
        }
        else {
            eprintln!("Error occured in get_bool in line: {next_line}");
            return Err(());
            }
    }
    else {
        eprintln!("Trying to get a bool in line {next_line}, but getting a non-bool");
        return Err(());
    }
}

pub fn get_color(color: &Color) -> f32 {
    for i in 1..15 {
        if color ==  &COLORS[i] {
            return i as f32;
        }
    }
    eprintln!("error in get_color!");
    return 0.0;
}

pub fn get_int(float: f32, next_line: &usize) -> Result<i32, ()> {
    if float.round() != float {
        eprintln!("in line: {next_line}, a parameter requires a integer with: {float}, but received a non-integer!");
        return Err(());
    }
    return Ok(float as i32);
}

pub fn get_number_or_bool(prefix: &Prefix, 
    rest: &str, 
    turtle : &Turtle, 
    variables: &HashMap<String, f32>, 
    next_line: &usize, 
    tokens: &mut std::str::SplitWhitespace
) -> Result<f32, ()> {

    // check if it is a variable containing a bool
    if prefix == &Prefix::Colon {
        if let Some(value) = variables.get(rest) {
            if value == &TRUE {
                return Ok(TRUE);
            }
            else if value == &FALSE {
                return Ok(FALSE);
            }
        }
        else {
            eprintln!("in line {next_line}, failed to retrieve a variable: {rest}");
            return Err(());
        }
    }

    // check if it is a bool value
    if is_bool(prefix) {
        return get_bool(prefix, next_line);
    }
    
    // check if it is a operator returning bool
    if prefix == &Prefix::OperatorBool {
        let value = math_calculation(tokens, rest, next_line, turtle, variables)?;
        return Ok(value);
    }
    // check if it is a number value or variable
    else if is_number(prefix) {
        return get_number(prefix, rest, turtle, variables, next_line, tokens);
    }
    else {
        eprintln!("in line: {next_line}, trying to get a number or bool with: {rest}, but failed");
        return Err(());
    }
}