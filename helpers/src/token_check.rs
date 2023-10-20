#[derive(PartialEq, Debug)]
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
            (Prefix::XCOR, "")
        } else if token == "YCOR" {
            (Prefix::YCOR, "")
        } else if token == "HEADING" {
            (Prefix::HEADING, "")
        } else if token == "COLOR" {
            (Prefix::COLOR, "")
        } else if token == "EQ"
            || token == "NE"
            || token == "GT"
            || token == "LT"
            || token == "AND"
            || token == "OR"
        {
            return (Prefix::OperatorBool, token);
        } else if token == "+" || token == "-" || token == "*" || token == "/" {
            return (Prefix::OperatorValue, token);
        } else if token.starts_with('\"') {
            if let Some(rest) = token.get(1..) {
                if rest.parse::<f32>().is_ok() {
                    return (Prefix::QuotationValue, rest);
                } else if rest == "TRUE" {
                    return (Prefix::TRUE, rest);
                } else if rest == "FALSE" {
                    return (Prefix::FALSE, rest);
                } else {
                    return (Prefix::QuotationVar, rest);
                }
            } else {
                return (Prefix::Wrong, "");
            }
        } else if token.starts_with(':') {
            if let Some(rest) = token.get(1..) {
                return (Prefix::Colon, rest);
            } else {
                return (Prefix::Wrong, "");
            }
        } else {
            return (Prefix::Wrong, "");
        }
    } else {
        (Prefix::Empty, "")
    }
}

// check if the token is potentially a number
// (if it is marked with :, the variable can be either a number or bool or not defined)
pub fn is_number(prefix: &Prefix) -> bool {
    match prefix {
        Prefix::XCOR => true,
        Prefix::YCOR => true,
        Prefix::HEADING => true,
        Prefix::COLOR => true,
        Prefix::QuotationValue => true,
        Prefix::QuotationVar => false,
        Prefix::Colon => true,
        Prefix::Empty => false,
        Prefix::Wrong => false,
        Prefix::TRUE => false,
        Prefix::FALSE => false,
        Prefix::OperatorBool => false,
        Prefix::OperatorValue => true,
    }
}

pub fn is_bool(prefix: &Prefix) -> bool {
    prefix == &Prefix::TRUE || prefix == &Prefix::FALSE
}

pub fn is_number_or_bool(prefix: &Prefix) -> bool {
    is_number(prefix) || is_bool(prefix)
}
