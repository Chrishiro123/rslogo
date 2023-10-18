#[derive(PartialEq, Debug)]
pub enum ConditionType {
    IF,
    WHILE,
}

#[derive(Debug)]
pub struct Condition {
    pub condition_type: ConditionType,
    pub index: usize,
    pub result: bool,
}

impl Condition {
    pub fn new(condition_type: ConditionType, index: usize, result: bool) -> Self {
        return Self { condition_type, index, result};
    }

    pub fn turn_off(&mut self) {
        self.result = false;
    }
}