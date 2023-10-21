pub struct Procedure {
    pub name: String,
    pub index: usize,
    arguments: Vec<String>,
}

#[derive(PartialEq)]
pub enum ProcCondi {
    Defining,
    Running,
    Out,
}

impl Procedure {
    pub fn new(name: String, index: usize, arguments: Vec<String>) -> Self {
        Self {
            name,
            index,
            arguments,
        }
    }
}

pub fn proc_match(procedures: &Vec<Procedure>, name: &str) -> Option<(usize, Vec<String>)> {
    for proc in procedures {
        if proc.name == name {
            return Some((proc.index, proc.arguments.clone()));
        }
    }
    None
}
