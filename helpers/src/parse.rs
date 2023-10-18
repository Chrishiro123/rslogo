use unsvg::Image;
use crate::conditions::Condition;
use crate::turtle::*;
use crate::token_check::*;
use crate::variables::*;
use crate::conditions::*;
use std::collections::HashMap;
use std::collections::VecDeque;

pub fn parse(mut tokens: std::str::SplitWhitespace,
     turtle: &mut Turtle, image: &mut Image,
      variables: &mut HashMap<String, f32>,
      _index: &mut usize,
      conditions: &mut VecDeque<Condition>,
    ) -> Result<(), ()> {
    // check if this line is in while or if loop
    // and dothing if not in any condition (empty conditions)
    if let Some(condition) = conditions.back() {
        // if condition is false, skip this line
        if condition.result == false {
            *_index += 1;
            return Ok(())
        }
    }

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

        Some("IF") => {
            //get next key word
            let res = tokens.next();
            let keyword: &str;
            match res {
                Some(value) => keyword = value,
                None => {
                    eprintln!("no parameter after IF keyword");
                    return Err(());
                },
            }
            if keyword != "EQ" {
                eprintln!("keyword after IF is not EQ!");
                return Err(());
            }
            //get the first parameter of IF
            let (prefix, rest) = prefix_check(tokens.next());
            if !is_number(&prefix) {
                eprintln!("some parameters in IF is not a number!");
                return Err(());
            }
            let value1: f32;
            match get_number(&prefix, rest, turtle, variables) {
                Ok(value) => value1 = value,
                Err(_) => {
                    eprintln!("Some parameters in IF are not defined variables!");
                    return Err(());
                },
            }
            //get the second parameter of IF
            let (prefix, rest) = prefix_check(tokens.next());
            if !is_number(&prefix) {
                eprintln!("some parameters in IF are not numbers!");
                return Err(());
            }
            let value2: f32;
            match get_number(&prefix, rest, turtle, variables) {
                Ok(value) => value2 = value,
                Err(_) => {
                    eprintln!("Some parameters in IF are not defined variables!");
                    return Err(());
                },
            }
            //check if condition is true
            if value1 == value2 {
                conditions.push_back(Condition::new(ConditionType::IF, _index.clone(), true));
            }
            // if condition is false
            else {
                conditions.push_back(Condition::new(ConditionType::IF, _index.clone(), false));
            }
            // check [ exist
            if let Some(value) = tokens.next() {
                if value == "[" {
                    ();
                }
                else {
                    eprintln!("missing [ in IF condition!");
                    return Err(());
                }
            }
            *_index += 1;
            return Ok(());
        },
        Some("WHILE") => {
            //get next key word
            let res = tokens.next();
            let keyword: &str;
            match res {
                Some(value) => keyword = value,
                None => {
                    eprintln!("no parameter after WHILE keyword");
                    return Err(());
                },
            }
            if keyword != "EQ" {
                eprintln!("keyword after WHILE is not EQ!");
                return Err(());
            }
            //get the first parameter of WHILE
            let (prefix, rest) = prefix_check(tokens.next());
            if !is_number(&prefix) {
                eprintln!("some parameters in WHILE is not a number!");
                return Err(());
            }
            let value1: f32;
            match get_number(&prefix, rest, turtle, variables) {
                Ok(value) => value1 = value,
                Err(_) => {
                    eprintln!("Some parameters in WHILE are not defined variables!");
                    return Err(());
                },
            }
            //get the second parameter of WHILE
            let (prefix, rest) = prefix_check(tokens.next());
            if !is_number(&prefix) {
                eprintln!("some parameters in WHILE are not numbers!");
                return Err(());
            }
            let value2: f32;
            match get_number(&prefix, rest, turtle, variables) {
                Ok(value) => value2 = value,
                Err(_) => {
                    eprintln!("Some parameters in WHILE are not defined variables!");
                    return Err(());
                },
            }
            // check if this is a new while, or just a repeat
            let mut repeat = false;
            match conditions.back_mut() {
                Some(value) => {
                    let last_condition = value;
                    if last_condition.index == *_index {
                        repeat = true;
                    }
                },
                // no existing conditions, this is a new "while"
                None => repeat = false,
            }
            //check if condition is true
            if value1 == value2 {
                // if it is a new "while", add in the Vecdequede
                if !repeat { 
                conditions.push_back(Condition::new(ConditionType::WHILE, _index.clone(), true));
                }
            }
            // if condition is false
            else {
                if !repeat {
                conditions.push_back(Condition::new(ConditionType::WHILE, _index.clone(), false));
                }
                else {
                    let last_condition = conditions.back_mut().unwrap();
                    last_condition.turn_off();
                }
            }
            // check [ exist
            if let Some(value) = tokens.next() {
                if value == "[" {
                    ();
                }
                else {
                    eprintln!("missing [ in WHILE condition!");
                    return Err(());
                }
            }
            *_index += 1;
            return Ok(());
        },
        Some("]") => {
            let condition: &Condition;
            match conditions.back() {
                //get the condition information
                Some(condi) => {
                    condition = condi;
                },
                // if not in any conditions but find ], report error
                None => {
                    eprintln!("found ] without matching [!");
                    return Err(());
                },
            }
            // if is in if condition, drop out the condition and continue next line
            if condition.condition_type == ConditionType::IF {
                conditions.pop_back();
                *_index += 1;
                return Ok(());
            }
            // if is in while condition, if the result is already false, pop out condition and continue
            // if the result is true, go back to the while check line again
            else {
                if condition.result == true {
                    *_index = condition.index;
                    return Ok(());
                }
                else {
                    *_index += 1;
                    return Ok(());
                }
            }

        }
        Some(&_) => return Err(()),
        None  => {
            *_index += 1;
            return Ok(());
        },
    }

}