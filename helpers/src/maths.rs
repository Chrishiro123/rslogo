use std::collections::{VecDeque, HashMap};
use crate::token_check::*;
use crate::turtle::Turtle;
use crate::variables::*;

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
        &Operator::EQ => true,
        &Operator::NE => true,
        &Operator::GT => true,
        &Operator::LT => true,
        &Operator::AND => true,
        &Operator::OR => true,
        &Operator::ADD => false,
        &Operator::SUB => false,
        &Operator::MUL => false,
        &Operator::DIV => false,
    }
}

//  check the token is a operator
pub fn is_operator(token: &str) -> bool {
    match token {
        "EQ" => true,
        "NE" => true,
        "GT" => true,
        "LT" => true,
        "AND" => true,
        "OR" => true,
        "+" => true,
        "-" => true,
        "*" => true,
        "/" => true,
        _ => false,
    }
}

// given the Option<&str> (not sure if there is a token)
// and get the meaning (i.e. operator)
// report error if no token inside or it is not a operator
pub fn get_operator(token: Option<&str>, next_line: &usize) -> Result<Operator, ()> {
    match token {
        Some(keyword) => {
            match keyword {
                "EQ" => Ok(Operator::EQ),
                "NE" => Ok(Operator::NE),
                "GT" => Ok(Operator::GT),
                "LT" => Ok(Operator::LT),
                "AND" => Ok(Operator::AND),
                "OR" => Ok(Operator::OR),
                "ADD" => Ok(Operator::ADD),
                "SUB" => Ok(Operator::SUB),
                "MUL" => Ok(Operator::MUL),
                "DIV" => Ok(Operator::DIV),
                _ => {
                    eprintln!("Get: {keyword}, in line: {next_line} when getting operator!");
                    return Err(());
                },
            }
        },
        None => {
            eprintln!("in line {next_line}, trying to get a operator but found nothing!");
            return Err(());
        },
    }
}

pub fn math_calculation(tokens: &mut std::str::SplitWhitespace, 
    first_operator: &str, 
    next_line: &usize, 
    turtle: &Turtle, 
    variables: &HashMap<String, f32>
) -> Result<f32, ()> {

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

            },
            None => {
                eprintln!("In line {next_line}, a math operator do not have enough operands");
                return Err(());
            },
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
            let operand1: f32;
            let operand2: f32;
            // get operand1
            match stack.pop_back() {
                Some(value1) => operand1 = value1,
                None => {
                    eprintln!("In line {next_line}, a math operator do not have enough operands");
                    return Err(());
                },
            }
            // get operand2
            match stack.pop_back() {
                Some(value2) => operand2 = value2,
                None => {
                    eprintln!("In line {next_line}, a math operator do not have enough operands");
                    return Err(());
                },
            }
            // do calculation
            let res = calculation(&operator, operand1, operand2, next_line)?;
            stack.push_back(res);
        }
        // store the value of token to stack if it is a operand
        else {
            let (prefix, rest) = prefix_check(Some(token));
            let value = get_number_or_bool(&prefix, rest, turtle, variables, next_line, tokens)?;
            stack.push_back(value);
        }
    }
    //get the final result
    let result = stack.pop_back();
    match result {
        Some(final_value) => {
            // stack have more than one result, which means too much operands
            if !stack.is_empty() {
                eprintln!("in line {next_line}, stack have more than one result, which means too much operands!");
                return Err(());
            }
            return Ok(final_value);
        },
        None => {
            // should not happen
            eprintln!("in line {next_line}, the calculation return no result!");
            return Err(());
        },
    }
} 

// a helper function doing calculation and also operands type check
pub fn calculation(operator: &Operator, operand1: f32, operand2: f32, next_line: &usize) -> Result<f32, ()> {
    match operator {
        &Operator::EQ => {
            if operand1 == operand2 {
                return Ok(TRUE);
            }
            else {
                return Ok(FALSE);
            }
        },
        &Operator::NE => {
            if operand1 != operand2 {
                return Ok(TRUE);
        }
            else {
                return Ok(FALSE);
            }
        },
        &Operator::GT => {
            if operand1 > operand2 {
                return Ok(TRUE);
            }
            else {
                return Ok(FALSE);
            }
        },
        &Operator::LT => {
            if operand1 < operand2 {
                return Ok(TRUE);
            }
            else {
                return Ok(FALSE);
            }
        },
        &Operator::AND => {
            if !is_bool_f32(operand1) || !is_bool_f32(operand2) {
                eprintln!("In {next_line}, AND operator get a non-bool operand!");
                return Err(());
            }
            if operand1 == TRUE && operand2 == TRUE {
                return Ok(TRUE);
            }
            else {
                return Ok(FALSE);
            }
        },
        &Operator::OR => {
            if !is_bool_f32(operand1) || !is_bool_f32(operand2) {
                eprintln!("In {next_line}, AND operator get a non-bool operand!");
                return Err(());
            }
            if operand1 == TRUE || operand2 == TRUE {
                return Ok(TRUE);
            }
            else {
                return Ok(FALSE);
            }
        },
        &Operator::ADD => {
            if is_bool_f32(operand1) || is_bool_f32(operand2) {
                eprintln!("In {next_line}, ADD operator get a bool operand!");
                return Err(());
            }
            else {
                return Ok(operand1 + operand2);
            }
        },
        &Operator::SUB => {
            if is_bool_f32(operand1) || is_bool_f32(operand2) {
                eprintln!("In {next_line}, ADD operator get a bool operand!");
                return Err(());
            }
            else {
                return Ok(operand1 - operand2);
            }
        },
        &Operator::MUL => {
            if is_bool_f32(operand1) || is_bool_f32(operand2) {
                eprintln!("In {next_line}, ADD operator get a bool operand!");
                return Err(());
            }
            else {
                return Ok(operand1 * operand2);
            }
        },
        &Operator::DIV => {
            if is_bool_f32(operand1) || is_bool_f32(operand2) {
                eprintln!("In line {next_line}, ADD operator get a bool operand!");
                return Err(());
            }
            else {
                if operand2 == 0.0 {
                    eprintln!("In line {next_line}, trying to divide by 0!")
                }
                return Ok(operand1 / operand2);
            }
        },
    }
}

// check if a f32 is the specified true/false f32 value
pub fn is_bool_f32(operand: f32) -> bool {
    if operand == TRUE || operand == FALSE {
        return true;
    }
    else {
        return false;
    }
}