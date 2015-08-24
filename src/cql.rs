
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
