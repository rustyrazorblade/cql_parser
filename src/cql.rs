use std::collections::HashMap;

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
    pub field: String,
    pub op: String,
    pub val: Value,
}

impl Predicate {
    pub fn new(field:String, op: String) -> Predicate {
        Predicate{field: field,
                    op: op,
                    val: Value::Placeholder}
    }
}

#[derive(Clone, Debug)]
pub enum Value {
    Int(isize),
    Float,
    UUID,
    Date,
    String,
    Expression(String),
    // in a prepared statement ?
    Placeholder,
    NamedPlaceHolder(String)
}


#[derive(Clone, Debug)]
pub struct InsertStatement {
    fields: Fields,
    kv: HashMap<String, Value>,
    table: String,
    lwt: bool
}

impl InsertStatement {
    // should merge the fields and the values to a hashmap
    pub fn new(table: String, fields: Fields, values: Vec<Value>, lwt: bool) -> InsertStatement {


        let kv = HashMap::new();
        // for field in fields.iter() {
        //
        // }
        InsertStatement{fields: fields, table:table, lwt: lwt, kv: kv}
    }
}

#[derive(Clone, Debug)]
pub struct SelectStatement {
    pub fields: Fields,
    pub table: String,
    pub where_clauses: Option<Vec<Predicate>>,
    pub order_by: Option<String>,
    pub limit: Option<isize>
}

impl SelectStatement {
    pub fn new(fields: Fields,
               table: String,
               where_clauses: Option<Vec<Predicate>>,
               order_by: Option<String>,
               limit: Option<isize>  )
               -> SelectStatement {

        SelectStatement{ fields: fields,
                         table:table,
                         where_clauses: where_clauses,
                         order_by: order_by,
                         limit: limit }
    }
}

#[derive(Clone, Debug)]
pub enum UsingOption {
    Timestamp(isize),
    TTL(isize)
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
