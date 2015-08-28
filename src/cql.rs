use std::collections::HashMap;

peg_file! cql("cql.rustpeg");



pub fn parse(stmt: &str) -> Result<ParsedCqlStatement, &str> {
    let result = match cql::cql_statement(stmt) {
        Ok(x) => Ok(x),
        _ => Err("meh")
    };
    result
}


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
    Named(Vec<String>),
    Count,
    None,
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
    UUID(String),
    Date,
    String(String),
    Expression(String),
    // in a prepared statement ?
    Placeholder,
    NamedPlaceHolder(String),
    Map(String),
    Set(String),
    List(String),
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
        // fields cannot be All
        // I don't know, do we panic?
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


#[test]
fn test_where() {
    assert!(cql::where_clauses("where term > ?").is_ok());
}

// counters
#[test]
fn test_counters() {
    assert!(cql::counter_op("blah = blah + 1").is_ok());
    assert!(cql::counter_op("blah = blah - 1").is_ok());
    assert!(cql::counter_op("blah = blah - ?").is_ok());
}

#[test]
fn test_timestamp() {
    assert!(cql::using_clause("using timestamp 60").is_ok());

}

#[test]
fn test_ttl() {
    assert!(cql::using_clause("using ttl 60").is_ok());
}

#[test]
fn test_multiple_where_clauses() {
    let p = cql::where_clauses("where k = ? and v = ?").unwrap();
}

#[test]
fn test_fields() {
    let parsed = cql::fields("name, age");
    assert!(parsed.is_ok());

    // should be a Fields enum
    match parsed.unwrap() {
        Fields::Named(v) => {
            assert!(v[0] == "name");
            println!("second field {}", v[1]);
            assert!(v[1] == "age");
        },
        _ => {
            panic!("Wrong type")
        }
    };

    let parsed = cql::fields_or_star("name, age").unwrap();
    let parsed = cql::fields_or_star("*").unwrap();

    match parsed {
        Fields::All => (),
        _ => { panic!("Wrong type returned")}
    };

}

#[test]
#[should_panic]
fn test_invalid_select() {
    cql::cql_statement("select from").unwrap();
}


#[test]
fn test_where_clause() {
    let p = cql::predicate("term > ?").unwrap();
    assert_eq!(p.field, "term");
    assert!(p.op == ">");

    if let Value::Placeholder = p.val {
    } else {
        panic!("Wrong type, expected placeholder")
    }
}

#[test]
fn test_comma_separated_values() {
    let vals = cql::comma_separated_values("?, ?, ?").unwrap();
    let vals = cql::comma_separated_values("1, 1, 1").unwrap();
}

#[test]
fn test_value_parsing() {
    cql::value("1").unwrap();
}

#[test]
fn test_uuid() {
    cql::uuid("de305d54-75b4-431b-adb2-eb6b9e546014").unwrap();
}

#[test]
fn test_quoted_string() {
    cql::quoted_string("'some text'").unwrap();
}

#[test]
fn test_quoted_string_with_escaped_quote() {
    cql::quoted_string("'some text ''bacon'' '").unwrap();
}

#[test]
fn test_predicate_string() {
    cql::predicate("k = 'test'").unwrap();
}

#[test]
fn test_where_string() {
    cql::where_clause("WHERE k = 'test'").unwrap();
}

#[test]
fn test_where_int() {
    cql::where_clause("where k = 1").unwrap();
}

#[test]
fn test_complex_where_clauses() {
    cql::where_clauses("WHERE event_type = 'myEvent' AND time > '2011-02-03' AND time <= '2012-01-01'").unwrap();
}
//WHERE event_type = 'myEvent' AND time > '2011-02-03' AND time <= '2012-01-01'

#[test]
fn test_less_than_eq() {
    let p = cql::predicate("time <= '2012-01-01'").unwrap();
}

#[test]
fn test_op() {
    let p = cql::op("<=").unwrap();
}

#[test]
fn test_map() {
    let p = cql::map_literal("{ 'fruit' : 'apple', 'band' : 'Beatles' }").unwrap();
    match p {
        Value::Map(m) => (),
        _ => { panic!("Wrong type, expected Map") }
    };
}

#[test]
fn test_set() {
    let p = cql::set_literal("{ 'fruit', 'apple', 'band', 'Beatles' }").unwrap();
    match p {
        Value::Set(m) => (),
        _ => { panic!("Wrong type, expected Map") }
    };
}

#[test]
fn test_collection_mutation() {
    let p = cql::collection_mutation("favs = favs +  { 'movie' : 'Cassablanca' }").unwrap();
    let p = cql::collection_mutation("scores = [ 12 ] + scores").unwrap();
}

#[test]
fn test_collection_update() {
    let p = cql::collection_update("favs[0] = 'bacon'").unwrap();
}

#[test]
fn test_count() {
    cql::count("count(*)").unwrap();
    cql::count("count(1)").unwrap();
}

// macro_rules! assert_enum {
//     () => {
//
//     };
// }


#[test]
fn test_delete_from_clause() {
    match cql::delete_from("from").unwrap() {
        Fields::None => {

        },
        _ => {
            panic!("wrong type");
        }
    };
    match cql::delete_from("field from").unwrap() {
        Fields::Named(x) => {

        },
        _ => {
            panic!("wrong type");
        }
    };
    match cql::delete_from("field, field2 from").unwrap() {
        Fields::Named(x) => {
            assert_eq!(2, x.len())  ;
        },
        _ => {
            panic!("wrong type");
        }
    };
}

fn test_in_clause() {

}
