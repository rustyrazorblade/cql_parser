
#[derive(Clone, Debug)]
pub enum ParsedCqlStatement {
    Select,
    Insert,
    Delete,
    Update
}


#[derive(Clone, Debug)]
pub struct Predicate {
    field: String,
    op: String
}

#[derive(Clone, Debug)]
pub enum Variable {
    Placeholder
}


#[derive(Clone, Debug)]
pub struct InsertStatement {
    fields: Vec<String>
}

impl InsertStatement {
    pub fn new() -> InsertStatement {
        InsertStatement{ fields: Vec::new() }
    }
}
