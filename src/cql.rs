
#[derive(Clone, Debug)]
pub enum ParsedCqlStatement {
    Select(SelectStatement),
    Insert(InsertStatement),
    Delete(DeleteStatement),
    Update(UpdateStatement),
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
    pub fn new(field:String, op: String) -> Predicate {
        Predicate{field: field,
                    op: op,
                    val: Variable::Placeholder}
    }
}

#[derive(Clone, Debug)]
pub enum Variable {
    Placeholder,
    NamedPlaceHolder(String)
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
    pub fields: Fields,
    pub table: String,
    pub where_clauses: Option<Vec<Predicate>>,
    pub order_by: Option<String>,
    pub limit: Option<String>
}

impl SelectStatement {
    pub fn new(fields: Fields,
               table: String,
               where_clauses: Option<Vec<Predicate>>,
               order_by: Option<String>,
               limit: Option<String>  )
               -> SelectStatement {

        SelectStatement{ fields: fields,
                         table:table,
                         where_clauses: where_clauses,
                         order_by: order_by,
                         limit: limit }
    }
}


#[derive(Clone, Debug)]
pub struct DeleteStatement {
    table: String
}

impl DeleteStatement {
    pub fn new(table: String) -> DeleteStatement {
        DeleteStatement{table:table}
    }
}



#[derive(Clone, Debug)]
pub struct UpdateStatement {
    table: String
}

impl UpdateStatement {
    pub fn new(table: String) -> UpdateStatement {
        UpdateStatement{table:table}
    }
}
