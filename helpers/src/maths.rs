use crate::err_handling::LogoError;
use crate::procedures::ProcCondi;
use crate::token_check::*;
use crate::turtle::Turtle;
use crate::variables::*;
use colored::Colorize;
use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
pub enum Operator {
    EQ,
    NE,
    GT,
    LT,
    AND,
    OR,
    ADD,
    SUB,
    MUL,
    DIV,
}

// check if the operator return type is a bool
pub fn is_return_bool(operator: &Operator) -> bool {
    match operator {
        Operator::EQ => true,
        Operator::NE => true,
        Operator::GT => true,
        Operator::LT => true,
        Operator::AND => true,
        Operator::OR => true,
        Operator::ADD => false,
        Operator::SUB => false,
        Operator::MUL => false,
        Operator::DIV => false,
    }
}

//  check the token is a operator
pub fn is_operator(token: &str) -> bool {
    matches!(
        token,
        "EQ" | "NE" | "GT" | "LT" | "AND" | "OR" | "+" | "-" | "*" | "/"
    )
}

// given the Option<&str> (not sure if there is a token)
// and get the meaning (i.e. operator)
// report error if no token inside or it is not a operator
pub fn get_operator(token: Option<&str>, next_line: &usize) -> Result<Operator, LogoError> {
    match token {
        Some(keyword) => match keyword {
            "EQ" => Ok(Operator::EQ),
            "NE" => Ok(Operator::NE),
            "GT" => Ok(Operator::GT),
            "LT" => Ok(Operator::LT),
            "AND" => Ok(Operator::AND),
            "OR" => Ok(Operator::OR),
            "+" => Ok(Operator::ADD),
            "-" => Ok(Operator::SUB),
            "*" => Ok(Operator::MUL),
            "/" => Ok(Operator::DIV),
            _ => Err(LogoError::new(format!(
                "Get: {}, in line: {} when getting operator!",
                keyword.red(),
                next_line.to_string().yellow()
            ))),
        },
        None => Err(LogoError::new(format!(
            "in line {}, trying to get a operator but found nothing!",
            next_line.to_string().yellow()
        ))),
    }
}

pub fn math_calculation(
    tokens: &mut std::str::SplitWhitespace,
    first_operator: &str,
    next_line: &usize,
    turtle: &Turtle,
    variables: &HashMap<String, f32>,
    proc_condi: &ProcCondi,
    proc_paras: &Option<HashMap<String, f32>>,
) -> Result<f32, LogoError> {
    // vecdeque for storing tokens
    let mut vec_deque = VecDeque::new();
    vec_deque.push_back(first_operator);
    let mut to_push = 2;
    while to_push > 0 {
        match tokens.next() {
            Some(token) => {
                //if it is a new operator, require 2 more operands
                if is_operator(token) {
                    to_push += 2;
                }
                to_push -= 1;
                vec_deque.push_back(token);
            }
            None => {
                return Err(LogoError::new(format!(
                    "In line {}, a math operator do not have enough operands",
                    next_line.to_string().yellow()
                )));
            }
        }
    }
    let iter_rev = vec_deque.into_iter().rev();
    // stack for storing numbers
    let mut stack: VecDeque<f32> = VecDeque::new();

    // reverse iterate the tokens
    for token in iter_rev {
        // calculate when operator found
        if is_operator(token) {
            let operator = get_operator(Some(token), next_line)?;
            // get operand1
            let operand1 = match stack.pop_back() {
                Some(value1) => value1,
                None => {
                    return Err(LogoError::new(format!(
                        "In line {}, a math operator do not have enough operands",
                        next_line.to_string().yellow()
                    )));
                }
            };
            // get operand2
            let operand2 = match stack.pop_back() {
                Some(value2) => value2,
                None => {
                    return Err(LogoError::new(format!(
                        "In line {}, a math operator do not have enough operands",
                        next_line.to_string().yellow()
                    )));
                }
            };
            // do calculation
            let res = calculation(&operator, operand1, operand2, next_line)?;
            stack.push_back(res);
        }
        // store the value of token to stack if it is a operand
        else {
            let (prefix, rest) = prefix_check(Some(token));
            let value = get_number_or_bool(
                &prefix, rest, turtle, variables, next_line, tokens, proc_condi, proc_paras,
            )?;
            stack.push_back(value);
        }
    }
    //get the final result
    let result = stack.pop_back();
    match result {
        Some(final_value) => {
            // stack have more than one result, which means too much operands
            if !stack.is_empty() {
                return Err(LogoError::new(format!(
                    "in line {}, stack have more than one result, which means too much operands!",
                    next_line.to_string().yellow()
                )));
            }
            Ok(final_value)
        }
        None => {
            // should not happen
            Err(LogoError::new(format!(
                "in line {}, the calculation return no result!",
                next_line.to_string().yellow()
            )))
        }
    }
}

// a helper function doing calculation and also operands type check
pub fn calculation(
    operator: &Operator,
    operand1: f32,
    operand2: f32,
    next_line: &usize,
) -> Result<f32, LogoError> {
    match operator {
        Operator::EQ => {
            if operand1 == operand2 {
                Ok(TRUE)
            } else {
                Ok(FALSE)
            }
        }
        Operator::NE => {
            if operand1 != operand2 {
                Ok(TRUE)
            } else {
                Ok(FALSE)
            }
        }
        Operator::GT => {
            if operand1 > operand2 {
                Ok(TRUE)
            } else {
                Ok(FALSE)
            }
        }
        Operator::LT => {
            if operand1 < operand2 {
                Ok(TRUE)
            } else {
                Ok(FALSE)
            }
        }
        Operator::AND => {
            if !is_bool_f32(operand1) || !is_bool_f32(operand2) {
                return Err(LogoError::new(format!(
                    "In {}, AND operator get a non-bool operand!",
                    next_line.to_string().yellow()
                )));
            }
            if operand1 == TRUE && operand2 == TRUE {
                Ok(TRUE)
            } else {
                Ok(FALSE)
            }
        }
        Operator::OR => {
            if !is_bool_f32(operand1) || !is_bool_f32(operand2) {
                return Err(LogoError::new(format!(
                    "In {}, AND operator get a non-bool operand!",
                    next_line.to_string().yellow()
                )));
            }
            if operand1 == TRUE || operand2 == TRUE {
                Ok(TRUE)
            } else {
                Ok(FALSE)
            }
        }
        Operator::ADD => {
            if is_bool_f32(operand1) || is_bool_f32(operand2) {
                Err(LogoError::new(format!(
                    "In {}, ADD operator get a bool operand!",
                    next_line.to_string().yellow()
                )))
            } else {
                Ok(operand1 + operand2)
            }
        }
        Operator::SUB => {
            if is_bool_f32(operand1) || is_bool_f32(operand2) {
                Err(LogoError::new(format!(
                    "In {}, ADD operator get a bool operand!",
                    next_line.to_string().yellow()
                )))
            } else {
                Ok(operand1 - operand2)
            }
        }
        Operator::MUL => {
            if is_bool_f32(operand1) || is_bool_f32(operand2) {
                Err(LogoError::new(format!(
                    "In {}, ADD operator get a bool operand!",
                    next_line.to_string().yellow()
                )))
            } else {
                Ok(operand1 * operand2)
            }
        }
        Operator::DIV => {
            if is_bool_f32(operand1) || is_bool_f32(operand2) {
                Err(LogoError::new(format!(
                    "In line {}, ADD operator get a bool operand!",
                    next_line.to_string().yellow()
                )))
            } else {
                if operand2 == 0.0 {
                    return Err(LogoError::new(format!(
                        "In line {}, trying to divide by 0!",
                        next_line.to_string().yellow()
                    )));
                }
                Ok(operand1 / operand2)
            }
        }
    }
}

// check if a f32 is the specified true/false f32 value
pub fn is_bool_f32(operand: f32) -> bool {
    operand == TRUE || operand == FALSE
}
