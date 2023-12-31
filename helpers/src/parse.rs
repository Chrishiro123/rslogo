use crate::conditions::Condition;
use crate::conditions::*;
use crate::err_handling::LogoError;
use crate::maths::*;
use crate::procedures::*;
use crate::token_check::*;
use crate::turtle::*;
use crate::variables::*;
use colored::Colorize;
use std::collections::HashMap;
use std::collections::VecDeque;
use unsvg::Image;

pub fn parse(
    mut tokens: std::str::SplitWhitespace,
    turtle: &mut Turtle,
    image: &mut Image,
    variables: &mut HashMap<String, f32>,
    _index: &mut usize,
    conditions: &mut VecDeque<Condition>,
    proc_condi: &mut ProcCondi,
    proc_paras: &mut Option<HashMap<String, f32>>,
    procedures: &mut Vec<Procedure>,
    return_index: &mut usize,
) -> Result<(), LogoError> {
    let first = tokens.next();
    // check if this line is in a procedure
    // if is and is during defining, do nothing
    if proc_condi == &mut ProcCondi::Defining && first != Some("END") {
        // if it is not END, do nothing
        //if it is END, will handle this in END branch
        *_index += 1;
        return Ok(());
    }

    // check if this line is in while or if loop
    // and dothing if not in any condition (empty conditions)
    if let Some(condition) = conditions.back() {
        // if condition is false, skip this line
        if !condition.result && first != Some("]") && first != Some("IF") && first != Some("WHILE")
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
        }

        Some("PENUP") => {
            *_index += 1;
            //check no extra parameter
            if tokens.next().is_some() {
                return Err(LogoError::new(format!(
                    "Too many parameters in {} in index: {}!",
                    "PENUP".blue(),
                    _index.to_string().yellow()
                )));
            }
            turtle.penup()
        }

        Some("PENDOWN") => {
            *_index += 1;
            //check no extra parameter
            if tokens.next().is_some() {
                return Err(LogoError::new(format!(
                    "Too many parameters in {}, in line: {}!",
                    "PENDOWN".blue(),
                    _index.to_string().yellow()
                )));
            }
            turtle.pendown()
        }

        Some("FORWARD") => {
            *_index += 1;
            //get the first parameter
            let (prefix, rest) = prefix_check(tokens.next());
            //check no extra parameter exists
            let numpixels = get_number(
                &prefix,
                rest,
                turtle,
                variables,
                _index,
                &mut tokens,
                proc_condi,
                proc_paras,
            )?;
            if tokens.next().is_some() {
                return Err(LogoError::new(format!(
                    "Too many parameters in {} in line: {}!",
                    "FORWARD".blue(),
                    _index.to_string().yellow()
                )));
            }
            turtle.forward(numpixels, image, _index)
        }

        Some("BACK") => {
            *_index += 1;
            //get the first parameter
            let (prefix, rest) = prefix_check(tokens.next());
            let numpixdels = get_number(
                &prefix,
                rest,
                turtle,
                variables,
                _index,
                &mut tokens,
                proc_condi,
                proc_paras,
            )?;
            //check no extra parameter exists
            if tokens.next().is_some() {
                return Err(LogoError::new(format!(
                    "Too many parameters in {} in line: {}!",
                    "BACK".green(),
                    _index.to_string().yellow()
                )));
            }
            turtle.back(numpixdels, image, _index)
        }

        Some("LEFT") => {
            *_index += 1;
            //get the first parameter
            let (prefix, rest) = prefix_check(tokens.next());
            let numpixels = get_number(
                &prefix,
                rest,
                turtle,
                variables,
                _index,
                &mut tokens,
                proc_condi,
                proc_paras,
            )?;
            //check no extra parameter exists
            if tokens.next().is_some() {
                return Err(LogoError::new(format!(
                    "Too many parameters in {}LEFT in line: {}!",
                    "LEFT".blue(),
                    _index.to_string().yellow()
                )));
            }
            turtle.left(numpixels, image, _index)
        }
        Some("RIGHT") => {
            *_index += 1;
            //get the first parameter
            let (prefix, rest) = prefix_check(tokens.next());
            let numpixdels = get_number(
                &prefix,
                rest,
                turtle,
                variables,
                _index,
                &mut tokens,
                proc_condi,
                proc_paras,
            )?;
            //check no extra parameter exists
            if tokens.next().is_some() {
                return Err(LogoError::new(format!(
                    "Too many parameters in {} in line: {}!",
                    "RIGHT".blue(),
                    _index.to_string().yellow()
                )));
            }
            turtle.right(numpixdels, image, _index)
        }

        Some("SETPENCOLOR") => {
            *_index += 1;
            //get the first parameter
            let (prefix, rest) = prefix_check(tokens.next());
            let value = get_number(
                &prefix,
                rest,
                turtle,
                variables,
                _index,
                &mut tokens,
                proc_condi,
                proc_paras,
            )?;
            //check the input is a integer
            let value_int = get_int(value, _index)?;
            //check no extra parameter exists
            if tokens.next().is_some() {
                return Err(LogoError::new(format!(
                    "Too many parameters in {} in line {}!",
                    "SETPENCOLOR".blue(),
                    _index.to_string().yellow()
                )));
            }
            turtle.setpencolor(value_int, _index)
        }

        Some("TURN") => {
            *_index += 1;
            //get the first parameter
            let (prefix, rest) = prefix_check(tokens.next());
            let value = get_number(
                &prefix,
                rest,
                turtle,
                variables,
                _index,
                &mut tokens,
                proc_condi,
                proc_paras,
            )?;
            //check the input is a integer
            let value_int = get_int(value, _index)?;
            //check no extra parameter exists
            if tokens.next().is_some() {
                return Err(LogoError::new(format!(
                    "Too many parameters in {}TURN in line: {}",
                    "TURN".blue(),
                    _index.to_string().yellow()
                )));
            }
            turtle.turn(value_int)
        }

        Some("SETHEADING") => {
            *_index += 1;
            //get the first parameter
            let (prefix, rest) = prefix_check(tokens.next());
            let value = get_number(
                &prefix,
                rest,
                turtle,
                variables,
                _index,
                &mut tokens,
                proc_condi,
                proc_paras,
            )?;
            let value_int = get_int(value, _index)?;
            //check no extra parameter exists
            if tokens.next().is_some() {
                return Err(LogoError::new(format!(
                    "Too many parameters in {}SETHEADING in line: {}!",
                    "SETHEADING".blue(),
                    _index.to_string().yellow()
                )));
            }
            turtle.setheading(value_int)
        }

        Some("SETX") => {
            *_index += 1;
            //get the first parameter
            let (prefix, rest) = prefix_check(tokens.next());
            let value = get_number(
                &prefix,
                rest,
                turtle,
                variables,
                _index,
                &mut tokens,
                proc_condi,
                proc_paras,
            )?;
            //check no extra parameter exists
            if tokens.next().is_some() {
                return Err(LogoError::new(format!(
                    "Too many parameters in {}SETX in line: {}!",
                    "SETX".blue(),
                    _index.to_string().yellow()
                )));
            }
            turtle.setx(value)
        }

        Some("SETY") => {
            *_index += 1;
            //get the first parameter
            let (prefix, rest) = prefix_check(tokens.next());
            let value = get_number(
                &prefix,
                rest,
                turtle,
                variables,
                _index,
                &mut tokens,
                proc_condi,
                proc_paras,
            )?;
            //check no extra parameter exists
            if tokens.next().is_some() {
                return Err(LogoError::new(format!(
                    "Too many parameters in {} in line: {}!",
                    "SETY".blue(),
                    _index.to_string().yellow()
                )));
            }
            turtle.sety(value)
        }

        Some("MAKE") => {
            *_index += 1;
            //get the first parameter
            let (prefix, variable_name) = prefix_check(tokens.next());
            //check grammer
            if prefix != Prefix::QuotationVar {
                Err(LogoError::new(format!(
                    "in line: {}, {} requires a variable as parameter!",
                    _index.to_string().yellow(),
                    "MAKE".blue()
                )))
            } else {
                //get the second parameter
                let (prefix, value_str) = prefix_check(tokens.next());
                let value = get_number_or_bool(
                    &prefix,
                    value_str,
                    turtle,
                    variables,
                    _index,
                    &mut tokens,
                    proc_condi,
                    proc_paras,
                )?;
                if tokens.next().is_some() {
                    return Err(LogoError::new(format!(
                        "Too many parameters in {} in line: {}!",
                        "MAKE".blue(),
                        _index.to_string().yellow()
                    )));
                }
                make(variables, variable_name, value);
                Ok(())
            }
        }
        Some("ADDASSIGN") => {
            *_index += 1;
            //get the first parameter
            let (prefix, rest1) = prefix_check(tokens.next());
            //check grammer
            if prefix != Prefix::QuotationVar {
                Err(LogoError::new(format!(
                    "in line: {}, {} requires a variable as parameter!",
                    _index.to_string().yellow(),
                    "ADDASSIGN".blue()
                )))
            } else {
                //get the second parameter
                let (prefix, rest2) = prefix_check(tokens.next());
                let value = get_number(
                    &prefix,
                    rest2,
                    turtle,
                    variables,
                    _index,
                    &mut tokens,
                    proc_condi,
                    proc_paras,
                )?;
                if tokens.next().is_some() {
                    return Err(LogoError::new(format!(
                        "Too many parameters in {} in line: {}!",
                        "ADDASSIGN".blue(),
                        _index.to_string().yellow()
                    )));
                }
                addassign(variables, rest1, value)
            }
        }

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
                    return Err(LogoError::new(format!(
                        "in line {}, no parameter exist after {}",
                        next_line.to_string().yellow(),
                        "IF".blue()
                    )));
                }
            };

            //get the first parameter of IF
            let (prefix, rest) = prefix_check(tokens.next());
            let value1 = get_number_or_bool(
                &prefix,
                rest,
                turtle,
                variables,
                &next_line,
                &mut tokens,
                proc_condi,
                proc_paras,
            )?;

            //get the second parameter of IF
            let (prefix, rest) = prefix_check(tokens.next());
            let value2 = get_number_or_bool(
                &prefix,
                rest,
                turtle,
                variables,
                &next_line,
                &mut tokens,
                proc_condi,
                proc_paras,
            )?;

            // start calculation
            let res = if !is_return_bool(&first_operator) {
                return Err(LogoError::new(format!(
                    "In line {}, The result of operator: {} of {} condition is not a bool!",
                    next_line.to_string().yellow(),
                    first_operator_str.red(),
                    "IF".blue()
                )));
            } else {
                calculation(&first_operator, value1, value2, &next_line)?
            };

            // push the result into condition vector
            if res == TRUE {
                conditions.push_back(Condition::new(ConditionType::IF, *_index, true));
            } else {
                conditions.push_back(Condition::new(ConditionType::IF, *_index, false));
            }

            // check [ exist
            if let Some(value) = tokens.next() {
                if value != "[" {
                    return Err(LogoError::new(format!(
                        "extra parameter in {} condition in line {}!",
                        "IF".yellow(),
                        next_line.to_string().blue()
                    )));
                }
            } else {
                return Err(LogoError::new(format!(
                    "missing {} in {} condition in line {}!",
                    "[".red(),
                    "IF".blue(),
                    next_line.to_string().yellow()
                )));
            }
            *_index += 1;
            Ok(())
        }
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
                    return Err(LogoError::new(format!(
                        "in line {}, no parameter exist after {}",
                        next_line.to_string().yellow(),
                        "WHILE".blue()
                    )));
                }
            };

            //get the first parameter of WHILE
            let (prefix, rest) = prefix_check(tokens.next());
            let value1 = get_number_or_bool(
                &prefix,
                rest,
                turtle,
                variables,
                &next_line,
                &mut tokens,
                proc_condi,
                proc_paras,
            )?;

            //get the second parameter of WHILE
            let (prefix, rest) = prefix_check(tokens.next());
            let value2 = get_number_or_bool(
                &prefix,
                rest,
                turtle,
                variables,
                &next_line,
                &mut tokens,
                proc_condi,
                proc_paras,
            )?;

            // check if this is a new while, or just a repeat
            let mut repeat = false;
            match conditions.back_mut() {
                Some(value) => {
                    let last_condition = value;
                    // it is a repeat since the code line is the same
                    if last_condition.index == *_index {
                        repeat = true;
                    }
                }
                // no existing conditions, this is a new "while"
                None => repeat = false,
            }

            // start calculation
            let while_result = if !is_return_bool(&first_operator) {
                return Err(LogoError::new(format!(
                    "In line {}, The result of operator: {} of {} condition is not a bool!",
                    next_line.to_string().yellow(),
                    first_operator_str.red(),
                    "WHILE".blue()
                )));
            } else {
                calculation(&first_operator, value1, value2, &next_line)?
            };

            if repeat {
                if while_result == FALSE {
                    let last_condition = conditions.back_mut().unwrap();
                    last_condition.turn_off();
                }
            } else if while_result == FALSE {
                conditions.push_back(Condition::new(ConditionType::WHILE, *_index, false));
            } else {
                conditions.push_back(Condition::new(ConditionType::WHILE, *_index, true));
            }

            // check [ exist
            if let Some(value) = tokens.next() {
                if value != "[" {
                    return Err(LogoError::new(format!(
                        "extra parameter in {} condition in line {}!",
                        "WHILE".blue(),
                        next_line.to_string().yellow()
                    )));
                }
            } else {
                eprintln!("missing ] in WHILE condition in line {next_line}");
            }
            *_index += 1;
            Ok(())
        }
        Some("]") => {
            let condition = match conditions.back() {
                //get the condition information
                Some(condi) => condi,
                // if not in any conditions but find ], report error
                None => {
                    return Err(LogoError::new(format!(
                        "found {} without matching {}!",
                        "]".red(),
                        "[".yellow()
                    )));
                }
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
            } else {
                conditions.pop_back();
                *_index += 1;
                Ok(())
            }
        }
        Some("TO") => {
            let next_line = *_index + 1;
            let proc_name = match tokens.next() {
                Some(value) => value,
                None => {
                    return Err(LogoError::new(format!(
                        "in line {}, get no procedure name when defining procedure!",
                        next_line.to_string().yellow()
                    )))
                }
            };
            let mut para_names = Vec::new();

            while let Some(name) = tokens.next() {
                // check all parameters start with " (no prefix error)
                if !name.starts_with('\"') {
                    return Err(LogoError::new(format!(
                        "in line {}, a procedure get a parameter whose prefix is not \"!",
                        next_line.to_string().yellow()
                    )));
                }
                para_names.push(name.get(1..).unwrap().to_string());
            }

            procedures.push(Procedure::new(proc_name.to_string(), *_index, para_names));
            *proc_condi = ProcCondi::Defining;
            *_index += 1;
            Ok(())
        }
        Some("END") => {
            let next_line = *_index + 1;
            match proc_condi {
                ProcCondi::Defining => {
                    *proc_condi = ProcCondi::Out;
                    *_index += 1;
                    Ok(())
                }
                ProcCondi::Running => {
                    *proc_condi = ProcCondi::Out;
                    *_index = *return_index;
                    Ok(())
                }
                ProcCondi::Out => Err(LogoError::new(format!(
                    "In line {}, a {} keyword found outside {}!",
                    next_line.to_string().yellow(),
                    "END".blue(),
                    "Procedure".blue()
                ))),
            }
        }
        // run a procedure or it is just a wrong argument
        Some(proc_name) => {
            let next_line = *_index + 1;
            // retrieve the start line of the procedure, and its parameter names
            let (start_line, para_names) = match proc_match(procedures, proc_name) {
                Some((value1, value2)) => (value1, value2),
                // no match, it is a invalid keyword
                None => {
                    return Err(LogoError::new(format!(
                        "The first argument {} in line {} is invalid!",
                        proc_name.red(),
                        next_line.to_string().yellow()
                    )));
                }
            };

            // create the map for parameters passed in the procedure
            let mut para_map: HashMap<String, f32> = HashMap::new();
            for name in para_names.iter() {
                let (prefix, rest) = prefix_check(tokens.next());
                let para_value = get_number(
                    &prefix,
                    rest,
                    turtle,
                    variables,
                    &next_line,
                    &mut tokens,
                    proc_condi,
                    proc_paras,
                )?;
                para_map.insert(name.to_string(), para_value);
            }
            // check if extra parameter exists
            if tokens.next().is_some() {
                return Err(LogoError::new(format!(
                    "Too many parameters in procedure {} in line: {}!",
                    proc_name.green(),
                    _index.to_string().yellow()
                )));
            }
            // update the parameter map for running procedure
            *proc_paras = Some(para_map);

            // running the procedure
            *proc_condi = ProcCondi::Running;
            // record the index to go back when procedure is finished
            *return_index = next_line;
            // jump to the body of the procedure
            *_index = start_line + 1;
            Ok(())
        }
        None => {
            *_index += 1;
            Ok(())
        }
    }
}
