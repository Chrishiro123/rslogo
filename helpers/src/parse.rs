use unsvg::Image;
use crate::turtle::*;
use crate::token_check::*;
use crate::variables::*;
use std::collections::HashMap;

pub fn parse(mut tokens: std::str::SplitWhitespace,
     turtle: &mut Turtle, image: &mut Image,
      variables: &mut HashMap<String, f32>,
      _index: &mut usize,
    ) -> Result<(), ()> {
    let first = tokens.next();
    match first {
        Some("//") => {
            *_index += 1;
            return Ok(())
        },
        Some("PENUP") => {
            *_index += 1;
            //check no extra parameter
            if tokens.next().is_none() {
                ();
            }
            else {
                eprintln!("Too many parameters in PENUP!");
                return Err(());                        
            }
            return turtle.penup()
        },
        Some("PENDOWN") => {
            *_index += 1;
            //check no extra parameter
            if tokens.next().is_none() {
                ();
            }
            else {
                eprintln!("Too many parameters in PENDOWN!");
                return Err(());                        
            }
            return turtle.pendown()
        },
        Some("FORWARD") => {
            *_index += 1;
            //get the first parameter
            let (prefix, rest) = prefix_check(tokens.next());
            //check no extra parameter exists
            if tokens.next().is_none() {
                ();
            }
            else {
                eprintln!("Too many parameters in FORWARD!");
                return Err(());                        
            }
            if is_number(&prefix) {
                let res = get_number(&prefix, rest, turtle, variables);
                match res {
                    Ok(value) => return turtle.forward(value, image),
                    Err(_e) => {
                        return Err(());      
                    }
                }
            }
            //report error if the parameter is not a number
            else {
                eprintln!("Wrong parameters in FORWARD!");
                return Err(());
            }
        }
        Some("BACK") => {
            *_index += 1;
            //get the first parameter
            let (prefix, rest) = prefix_check(tokens.next());
            //check no extra parameter exists
            if tokens.next().is_none() {
                ();
            }
            else {
                eprintln!("Too many parameters in BACK!");
                return Err(());                        
            }
            if is_number(&prefix) {
                let res = get_number(&prefix, rest, turtle, variables);
                match res {
                    Ok(value) => return turtle.back(value, image),
                    Err(_e) => {
                        return Err(());      
                    }
                }
            }
            //report error if the parameter is not a number
            else {
                eprintln!("Wrong parameters in BACK!");
                return Err(());
            }
        },
        Some("LEFT") => {
            *_index += 1;
            //get the first parameter
            let (prefix, rest) = prefix_check(tokens.next());
            //check no extra parameter exists
            if tokens.next().is_none() {
                ();
            }
            else {
                eprintln!("Too many parameters in LEFT!");
                return Err(());                        
            }
            if is_number(&prefix) {
                let res = get_number(&prefix, rest, turtle, variables);
                match res {
                    Ok(value) => return turtle.left(value, image),
                    Err(_e) => {
                        return Err(());      
                    }
                }
            }
            //report error if the parameter is not a number
            else {
                eprintln!("Wrong parameters in LEFT!");
                return Err(());
            }
        },
        Some("RIGHT") => {
            *_index += 1;
            //get the first parameter
            let (prefix, rest) = prefix_check(tokens.next());
            //check no extra parameter exists
            if tokens.next().is_none() {
                ();
            }
            else {
                eprintln!("Too many parameters in RIGHT!");
                return Err(());                        
            }
            if is_number(&prefix) {
                let res = get_number(&prefix, rest, turtle, variables);
                match res {
                    Ok(value) => return turtle.right(value, image),
                    Err(_e) => {
                        return Err(());      
                    }
                }
            }
            //report error if the parameter is not a number
            else {
                eprintln!("Wrong parameters in RIGHT!");
                return Err(());
            }
        },
        Some("SETPENCOLOR") => {
            *_index += 1;
            //get the first parameter
            let (prefix, rest) = prefix_check(tokens.next());
            //check no extra parameter exists
            if tokens.next().is_none() {
                ();
            }
            else {
                eprintln!("Too many parameters in SETPENCOLOR!");
                return Err(());                        
            }
            if is_number(&prefix) {
                let res = get_number(&prefix, rest, turtle, variables);
                match res {
                    Ok(value) => {
                        //check the input is a integer
                        if value.round() != value {
                            eprintln!("SETPENCOLOR need a integer!");
                            return Err(());
                        }
                        return turtle.setpencolor(value as i32)
                    },
                    Err(_e) => {
                        return Err(());      
                    }
                }
            }
            //report error if the parameter is not a number
            else {
                eprintln!("Wrong parameters in SETPENCOLOR!");
                return Err(());
            }
        }
        Some("TURN") => {
            *_index += 1;
            //get the first parameter
            let (prefix, rest) = prefix_check(tokens.next());
            //check no extra parameter exists
            if tokens.next().is_none() {
                ();
            }
            else {
                eprintln!("Too many parameters in TURN!");
                return Err(());                        
            }
            if is_number(&prefix) {
                let res = get_number(&prefix, rest, turtle, variables);
                match res {
                    Ok(value) => {
                        //check the input is a integer
                        if value.round() != value {
                            eprintln!("TURN need a integer!");
                            return Err(());
                        }
                        return turtle.turn(value as i32)
                    },
                    Err(_e) => {
                        return Err(());      
                    }
                }
            }
            //report error if the parameter is not a number
            else {
                eprintln!("Wrong parameters in TURN!");
                return Err(());
            }
        }
        Some("SETHEADING") => {
            *_index += 1;
            //get the first parameter
            let (prefix, rest) = prefix_check(tokens.next());
            //check no extra parameter exists
            if tokens.next().is_none() {
                ();
            }
            else {
                eprintln!("Too many parameters in SETHEADING!");
                return Err(());                        
            }
            if is_number(&prefix) {
                let res = get_number(&prefix, rest, turtle, variables);
                match res {
                    Ok(value) => {
                        //check the input is a integer
                        if value.round() != value {
                            eprintln!("SETHEADING need a integer!");
                            return Err(());
                        }
                        return turtle.setheading(value as i32)
                    },
                    Err(_e) => {
                        return Err(());      
                    }
                }
            }
            //report error if the parameter is not a number
            else {
                eprintln!("Wrong parameters in TURN!");
                return Err(());
            }
        }
        Some("SETX") => {
            *_index += 1;
            //get the first parameter
            let (prefix, rest) = prefix_check(tokens.next());
            //check no extra parameter exists
            if tokens.next().is_none() {
                ();
            }
            else {
                eprintln!("Too many parameters in SETX!");
                return Err(());                        
            }
            if is_number(&prefix) {
                let res = get_number(&prefix, rest, turtle, variables);
                match res {
                    Ok(value) => {
                        return turtle.setx(value);
                    },
                    Err(_e) => {
                        return Err(());      
                    }
                }
            }
            //report error if the parameter is not a number
            else {
                eprintln!("Wrong parameters in TURN!");
                return Err(());
            }
        }
        Some("SETY") => {
            *_index += 1;
            //get the first parameter
            let (prefix, rest) = prefix_check(tokens.next());
            //check no extra parameter exists
            if tokens.next().is_none() {
                ();
            }
            else {
                eprintln!("Too many parameters in SETY!");
                return Err(());                        
            }
            if is_number(&prefix) {
                let res = get_number(&prefix, rest, turtle, variables);
                match res {
                    Ok(value) => {
                        return turtle.sety(value);
                    },
                    Err(_e) => {
                        return Err(());      
                    }
                }
            }
            //report error if the parameter is not a number
            else {
                eprintln!("Wrong parameters in SETY!");
                return Err(());
            }
        }
        Some("MAKE") => {
            *_index += 1;
            //get the first parameter
            let (prefix, rest1) = prefix_check(tokens.next());
            //check grammer
            if prefix != Prefix::QuotationVar {
                eprintln!("MAKE requires a variable as parameter!");
                return Err(());
            }
            else {
                //get the second parameter
                let (prefix, rest2) = prefix_check(tokens.next());
                if is_number(&prefix) {
                    let value = get_number(&prefix, rest2, turtle, variables);
                    match value {
                        Ok(value) => {
                            make(variables, rest1, value);
                            return Ok(());
                        },
                        Err(_e) => {
                            eprintln!("Wrong parameter in MAKE!");
                            return Err(());
                        },
                    }
                }
                else if is_bool(&prefix) {
                    make(variables, rest1, get_bool(&prefix));
                    return Ok(());
                }
                else {
                    eprintln!("The second MAKE parameter is not not valid!");
                    return Err(());
                }
            }
        }
        Some("ADDASSIGN") => {
            *_index += 1;
            //get the first parameter
            let (prefix, rest1) = prefix_check(tokens.next());
            //check grammer
            if prefix != Prefix::QuotationVar {
                eprintln!("ADDASSIGN requires a variable as parameter!");
                return Err(());
            }
            else {
                //get the second parameter
                let (prefix, rest2) = prefix_check(tokens.next());
                if is_number(&prefix) {
                    let value = get_number(&prefix, rest2, turtle, variables);
                    match value {
                        Ok(value) => {
                            return addassign(variables, rest1, value);
                        },
                        Err(_e) => {
                            eprintln!("Wrong parameter in ADDASSIGN!");
                            return Err(());
                        },
                    }
                }
                else {
                    eprintln!("The second ADDASSIGN parameter is not a number!");
                    return Err(());
                }
            }
        }

        Some(&_) => todo!(),
        None  => {
            *_index += 1;
            return Ok(());
        },
    }

}