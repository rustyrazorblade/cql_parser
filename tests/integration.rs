extern crate cql_parser;

use cql_parser::cql::parse;

#[test]
fn test_legal_queries() {
    let queries = include_str!("passing.cql").split("\n");

    for query in queries {

    }

}
