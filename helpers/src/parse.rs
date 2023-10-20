use unsvg::Image;
use crate::conditions::Condition;
use crate::turtle::*;
use crate::token_check::*;
use crate::variables::*;
use crate::conditions::*;
use crate::maths::*;
use std::collections::HashMap;
use std::collections::VecDeque;

pub fn parse(mut tokens: std::str::SplitWhitespace,
     turtle: &mut Turtle, image: &mut Image,
      variables: &mut HashMap<String, f32>,
      _index: &mut usize,
      conditions: &mut VecDeque<Condition>,
    ) -> Result<(), ()> {
    let first = tokens.next();
    // check if this line is in while or if loop
    // and dothing if not in any condition (empty conditions)
    if let Some(condition) = conditions.back() {
        // if condition is false, skip this line
        if !condition.result 
            && first != Some("]")
            && first != Some("IF")
            && first != Some("WHILE")
            {
            *_index += 1;
            return Ok(());
        }
    }

    // match the first token (the function name)
    match first {
        Some("//") => {
            *_index += 1;
            Ok(())
        },

        Some("PENUP") => {
            *_index += 1;
            //check no extra parameter
            if tokens.next().is_some() {
                eprintln!("Too many parameters in PENUP in index: {_index}!");
                return Err(());                        
            }
            turtle.penup()
        },

        Some("PENDOWN") => {
            *_index += 1;
            //check no extra parameter
            if tokens.next().is_some() {
                eprintln!("Too many parameters in PENDOWN in line: {_index}!");
                return Err(());                        
            }
            turtle.pendown()
        },

        Some("FORWARD") => {
            *_index += 1;
            //get the first parameter
            let (prefix, rest) = prefix_check(tokens.next());
            //check no extra parameter exists
            let numpixels = get_number(&prefix, rest, turtle, variables, _index, &mut tokens)?;
            if tokens.next().is_some() {
                eprintln!("Too many parameters in FORWARD in line: {_index}!");
                return Err(());                        
            }
            turtle.forward(numpixels, image)
        },

        Some("BACK") => {
            *_index += 1;
            //get the first parameter
            let (prefix, rest) = prefix_check(tokens.next());
            let numpixdels = get_number(&prefix, rest, turtle, variables, _index, &mut tokens)?;
            //check no extra parameter exists
            if tokens.next().is_some() {
                eprintln!("Too many parameters in BACK in line: {_index}!");
                return Err(());                        
            }
            turtle.back(numpixdels, image)
        },

        Some("LEFT") => {
            *_index += 1;
            //get the first parameter
            let (prefix, rest) = prefix_check(tokens.next());
            let numpixels = get_number(&prefix, rest, turtle, variables, _index, &mut tokens)?;
            //check no extra parameter exists
            if tokens.next().is_some() {
                eprintln!("Too many parameters in LEFT in line: {_index}!");
                return Err(());                        
            }
            turtle.left(numpixels, image)
        },
        Some("RIGHT") => {
            *_index += 1;
            //get the first parameter
            let (prefix, rest) = prefix_check(tokens.next());
            let numpixdels = get_number(&prefix, rest, turtle, variables, _index, &mut tokens)?;
            //check no extra parameter exists
            if tokens.next().is_some() {
                eprintln!("Too many parameters in RIGHT in line: {_index}!");
                return Err(());                        
            }
            turtle.right(numpixdels, image)
        },

        Some("SETPENCOLOR") => {
            *_index += 1;
            //get the first parameter
            let (prefix, rest) = prefix_check(tokens.next());
            let value = get_number(&prefix, rest, turtle, variables, _index, &mut tokens)?;
            //check the input is a integer
            let value_int = get_int(value, _index)?;
            //check no extra parameter exists
            if tokens.next().is_some() {
                eprintln!("Too many parameters in SETPENCOLOR in line {_index}!");
                return Err(());                        
            }
                turtle.setpencolor(value_int)
        },

        Some("TURN") => {
            *_index += 1;
            //get the first parameter
            let (prefix, rest) = prefix_check(tokens.next());
            let value = get_number(&prefix, rest, turtle, variables, _index, &mut tokens)?;
            //check the input is a integer
            let value_int = get_int(value, _index)?;
            //check no extra parameter exists
            if tokens.next().is_some() {
                eprintln!("Too many parameters in TURN in line: {_index}");
                return Err(());                        
            }
            turtle.turn(value_int)
        },

        Some("SETHEADING") => {
            *_index += 1;
            //get the first parameter
            let (prefix, rest) = prefix_check(tokens.next());
            let value = get_number(&prefix, rest, turtle, variables, _index, &mut tokens)?;
            let value_int = get_int(value, _index)?;
            //check no extra parameter exists
            if tokens.next().is_some() {
                eprintln!("Too many parameters in SETHEADING in line: {_index}!");
                return Err(());                        
            }
            turtle.setheading(value_int)
        }

        Some("SETX") => {
            *_index += 1;
            //get the first parameter
            let (prefix, rest) = prefix_check(tokens.next());
            let value = get_number(&prefix, rest, turtle, variables, _index, &mut tokens)?;
            //check no extra parameter exists
            if tokens.next().is_some() {
                eprintln!("Too many parameters in SETX in line: {_index}!");
                return Err(());                        
            }
            turtle.setx(value)
        },

        Some("SETY") => {
            *_index += 1;
            //get the first parameter
            let (prefix, rest) = prefix_check(tokens.next());
            let value = get_number(&prefix, rest, turtle, variables, _index, &mut tokens)?;
            //check no extra parameter exists
            if tokens.next().is_some() {
                eprintln!("Too many parameters in SETY in line: {_index}!");
                return Err(());                        
            }
            turtle.sety(value)
        },

        Some("MAKE") => {
            *_index += 1;
            //get the first parameter
            let (prefix, variable_name) = prefix_check(tokens.next());
            //check grammer
            if prefix != Prefix::QuotationVar {
                eprintln!("in line: {_index}, MAKE requires a variable as parameter!");
                Err(())
            }
            else {
                //get the second parameter
                let (prefix, value_str) = prefix_check(tokens.next());
                let value = get_number_or_bool(&prefix, value_str, turtle, variables, _index, &mut tokens)?;
                if tokens.next().is_some() {
                    eprintln!("Too many parameters in MAKE in line: {_index}!");
                    return Err(());                        
                }
                make(variables, variable_name, value);
                Ok(())
            }
        },
        Some("ADDASSIGN") => {
            *_index += 1;
            //get the first parameter
            let (prefix, rest1) = prefix_check(tokens.next());
            //check grammer
            if prefix != Prefix::QuotationVar {
                eprintln!("in line: {_index}, ADDASSIGN requires a variable as parameter!");
                Err(())
            }
            else {
                //get the second parameter
                let (prefix, rest2) = prefix_check(tokens.next());
                let value = get_number(&prefix, rest2, turtle, variables, _index, &mut tokens)?;
                if tokens.next().is_some() {
                    eprintln!("Too many parameters in ADDASSIGN in line: {_index}!");
                    return Err(());                        
                }
                addassign(variables, rest1, value)
            }
        },

        Some("IF") => {
            // check if it is in another IF/WHILE bracket
            if let Some(value) = conditions.back() {
                if !value.result {
                    // if is in another false bracket, do not execute, but just add a new false condition in (to match its ])
                    // if is in a true bracket, execute as normal
                    conditions.push_back(Condition::new(ConditionType::IF, *_index, false));
                    *_index += 1;
                    return Ok(());
                }
            }

            //get operator
            let res = tokens.next();
            let next_line = *_index + 1;
            let first_operator = get_operator(res, &next_line)?;
            let first_operator_str = match res {
                Some(value) => value,
                None => {
                    eprintln!("in line {next_line}, no parameter exist after IF");
                    return Err(());
                }
            };

            //get the first parameter of IF
            let (prefix, rest) = prefix_check(tokens.next());
            let value1 = get_number_or_bool(&prefix, rest, turtle, variables, &next_line, &mut tokens)?;

            //get the second parameter of IF
            let (prefix, rest) = prefix_check(tokens.next());
            let value2 = get_number_or_bool(&prefix, rest, turtle, variables, &next_line, &mut tokens)?;

            // start calculation
            let res = if !is_return_bool(&first_operator) {
                eprintln!("In line {next_line}, The result of operator: {first_operator_str} of IF condition is not a bool!");
                return Err(());
            }
            else {
                calculation(&first_operator, value1, value2, &next_line)?
            };

            // push the result into condition vector
            if res == TRUE {
                conditions.push_back(Condition::new(ConditionType::IF, *_index, true));
            }
            else {
                conditions.push_back(Condition::new(ConditionType::IF, *_index, false));
            }

            // check [ exist
            if let Some(value) = tokens.next() {
                if value != "[" {
                    eprintln!("extra parameter in IF condition in line {next_line}!");
                    return Err(());
                }
            }
            else {
                eprintln!("missing [ in IF condition in line {next_line}!");
                return Err(());
            }
            *_index += 1;
            Ok(())
        },
        Some("WHILE") => {
            // check if it is in another IF/WHILE bracket
            if let Some(value) = conditions.back() {
                if !value.result {
                    // if is in another false bracket, do not execute, but just add a new false condition in (to match its ])
                    // if is in a true bracket, execute as normal
                    conditions.push_back(Condition::new(ConditionType::WHILE, *_index, false));
                    *_index += 1;
                    return Ok(());
                }
            }

            //get next key word
            let res = tokens.next();
            let next_line = *_index + 1;
            let first_operator = get_operator(res, &next_line)?;
            let first_operator_str = match res {
                Some(value) => value,
                None => {
                    eprintln!("in line {next_line}, no parameter exist after WHILE");
                    return Err(());
                }
            };

            //get the first parameter of WHILE
            let (prefix, rest) = prefix_check(tokens.next());
            let value1 = get_number_or_bool(&prefix, rest, turtle, variables, &next_line, &mut tokens)?;

            //get the second parameter of WHILE
            let (prefix, rest) = prefix_check(tokens.next());
            let value2 = get_number_or_bool(&prefix, rest, turtle, variables, &next_line, &mut tokens)?;

            // check if this is a new while, or just a repeat
            let mut repeat = false;
            match conditions.back_mut() {
                Some(value) => {
                    let last_condition = value;
                    // it is a repeat since the code line is the same
                    if last_condition.index == *_index {
                        repeat = true;
                    }
                },
                // no existing conditions, this is a new "while"
                None => repeat = false,
            }

            // start calculation
            let while_result = if !is_return_bool(&first_operator) {
                eprintln!("In line {next_line}, The result of operator: {first_operator_str} of WHILE condition is not a bool!");
                return Err(());
            }
            else {
                calculation(&first_operator, value1, value2, &next_line)?
            };

            if repeat {
                if while_result == FALSE {
                    let last_condition = conditions.back_mut().unwrap();
                    last_condition.turn_off();
                }
            }
            else if while_result == FALSE {
                conditions.push_back(Condition::new(ConditionType::WHILE, *_index, false));
            }
            else {
                conditions.push_back(Condition::new(ConditionType::WHILE, *_index, true));
            }

            // check [ exist
            if let Some(value) = tokens.next() {
                if value != "[" {
                    eprintln!("extra parameter in WHILE condition in line {next_line}!");
                    return Err(());
                }
            }
            else {
                eprintln!("missing ] in WHILE condition in line {next_line}");
            }
            *_index += 1;
            Ok(())
        },
        Some("]") => {
            let condition = match conditions.back() {
                //get the condition information
                Some(condi) => {
                    condi
                },
                // if not in any conditions but find ], report error
                None => {
                    eprintln!("found ] without matching [!");
                    return Err(());
                },
            };
            // if is in if condition, drop out the condition and continue next line
            if condition.condition_type == ConditionType::IF {
                conditions.pop_back();
                *_index += 1;
                Ok(())
            }
            // if is in while condition, if the result is already false, pop out condition and continue
            // if the result is true, go back to the while check line again
            else if condition.result {
                *_index = condition.index;
                Ok(())
                }
            else {
                conditions.pop_back();
                *_index += 1;
                Ok(())
            }
        }
        Some(&_) => Err(()),
        None  => {
            *_index += 1;
            Ok(())
        },
    }

}