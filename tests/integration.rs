extern crate cql_parser;

use cql_parser::cql::parse;

#[test]
fn test_legal_queries() {
    let queries = include_str!("passing.cql").split("\n");

    for query in queries {
        // skip blank lines and lines that are a comment
        if query.trim() == "" {
            continue;
        }
        if query[0..2].to_string() == "//" {
            continue;
        }
        let parsed = parse(query);
        match parsed {
            Ok(stmt) => (),
            Err(x) => {
                panic!("Error parsing {}", query);
            }
        };
    }

}
