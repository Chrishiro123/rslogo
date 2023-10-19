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