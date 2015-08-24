
#[derive(Clone, Debug)]
pub enum ParsedCqlStatement {
    Select(SelectStatement),
    Insert(InsertStatement),
    Delete,
    Update
}

#[derive(Clone, Debug)]
pub enum Fields {
    All,
    Named(Vec<String>)
}

#[derive(Clone, Debug)]
pub struct Predicate {
    field: String,
    op: String,
    val: Variable,
}

impl Predicate {
    pub fn new(field:String) -> Predicate {
        Predicate{field: field,
                    op: "test".to_string(),
                    val: Variable::Placeholder}
    }
}

#[derive(Clone, Debug)]
pub enum Variable {
    Placeholder
}

#[derive(Clone, Debug)]
pub enum Value {
    Int,
    Float,
    UUID,
    Date,
    Expression(String)
}


#[derive(Clone, Debug)]
pub struct InsertStatement {
    fields: Fields,
    table: String
}

impl InsertStatement {
    // should merge the fields and the values to a hashmap
    pub fn new(fields: Fields, table: String) -> InsertStatement {

        InsertStatement{fields: fields, table:table}
    }
}

#[derive(Clone, Debug)]
pub struct SelectStatement {
    fields: Fields,
    table: String
}

impl SelectStatement {
    pub fn new(fields: Fields, table: String) -> SelectStatement {
        SelectStatement{ fields: fields, table:table}
    }
}
