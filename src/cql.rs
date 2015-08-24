
#[derive(Clone, Debug)]
pub enum ParsedCqlStatement {
    Select(SelectStatement),
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

#[derive(Clone, Debug)]
pub struct SelectStatement {
    fields: Vec<String>
}

impl SelectStatement {
    pub fn new() -> SelectStatement {
        SelectStatement{ fields: Vec::new() }
    }
}
