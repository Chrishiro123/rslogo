#[derive(PartialEq)]


#[derive(Debug)]
pub enum Prefix {
    XCOR,
    YCOR,
    HEADING,
    COLOR,
    QuotationValue,
    QuotationVar,
    Colon,
    Empty,
    Wrong,
    TRUE,
    FALSE,
    OperatorBool,
    OperatorValue,
}

// analyse the token if exist, return (Prefix::Empty, "") if empty
pub fn prefix_check(token_or_not: Option<&str>) -> (Prefix, &str) {
    if let Some(token) = token_or_not {
        if token == "XCOR" {
            return (Prefix::XCOR, "");
        }
        else if token == "YCOR" {
            return (Prefix::YCOR, "");
        }
        else if token == "HEADING" {
            return (Prefix::HEADING, "");
        }
        else if token == "COLOR" {
            return (Prefix::COLOR, "");
        }
        else if token == "EQ"
            || token == "NE"
            || token == "GT"
            || token == "LT"
            || token == "AND"
            || token == "OR"
        {
            return (Prefix::OperatorBool, token);
        }
        else if token == "+"
            || token == "-"
            || token == "*"
            || token == "/"
        {
            return (Prefix::OperatorValue, token);    
        }
        else if token.chars().next().unwrap() == '"' {
            if let Some(rest) = token.get(1..) {
                if rest.parse::<f32>().is_ok() {
                    return (Prefix::QuotationValue, rest);
                }
                else if rest == "TRUE" {
                    return (Prefix::TRUE, rest);
                }
                else if rest == "FALSE" {
                    return (Prefix::FALSE, rest)
                }
                else {
                    return (Prefix::QuotationVar, rest);
                }
            }
            else {
                return (Prefix::Wrong, "");
            }
        }
        else if token.chars().next().unwrap() == ':' {
            if let Some(rest) = token.get(1..) {
                return (Prefix::Colon, rest);
            }
            else {
                return (Prefix::Wrong, "");
            }
        }
        else {
            return (Prefix::Wrong, "");
        }
    }
    else {
        return (Prefix::Empty, "");
    }
}

// check if the token is potentially a number 
// (if it is marked with :, the variable can be either a number or bool or not defined)
pub fn is_number(prefix: &Prefix) -> bool {
    match prefix {
        &Prefix::XCOR => return true,
        &Prefix::YCOR => return true,
        &Prefix::HEADING => return true,
        &Prefix::COLOR => return true,
        &Prefix::QuotationValue => return true,
        &Prefix::QuotationVar => return false,
        &Prefix::Colon => return true,
        &Prefix::Empty => return false,
        &Prefix::Wrong => return false,
        &Prefix::TRUE => return false,
        &Prefix::FALSE => return false,
        &Prefix::OperatorBool => return false,
        &Prefix::OperatorValue => return true,
    }
}

pub fn is_bool(prefix: &Prefix) -> bool {
    if prefix == &Prefix::TRUE || prefix == &Prefix::FALSE {
        return true;
    }
    else {
        return false;
    }
}

pub fn is_number_or_bool(prefix: &Prefix) -> bool {
    return is_number(prefix) || is_bool(prefix);
}
