use std::collections::HashMap;
use crate::{token_check::Prefix, turtle::Turtle};

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

pub fn get_number(prefix: &Prefix, rest: &str, turtle : &Turtle, variables: &mut HashMap<String, f32>) -> Result<f32, ()> {
    match prefix {
        &Prefix::XCOR => Ok(turtle.x),
        &Prefix::YCOR => Ok(turtle.y),
        &Prefix::HEADING => Ok(turtle.direction as f32),
        &Prefix::QuotationValue => Ok(rest.parse::<f32>().unwrap()),
        &Prefix::Colon => {
            match variables.get(rest) {
                Some(value) => Ok(*value),
                None => {
                    eprintln!("trying to retrieve a non-existing variable: {rest}!");
                    Err(())
                },
            }
        },
        //won't happen here
        _ => Err(()),
    }
}

pub fn get_bool(prefix: &Prefix) -> f32 {
    if prefix == &Prefix::TRUE {
        return TRUE;
    }
    else if prefix == &Prefix::FALSE{
        return FALSE;
    }
    else {
        eprintln!("Error occured in get_bool");
        return 0.0;
    }
}