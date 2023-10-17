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
}

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