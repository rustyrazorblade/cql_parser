extern crate cql_parser;

use cql_parser::cql::parse;

#[test]
fn test_legal_queries() {
    let queries = include_str!("passing.cql").split("\n");
    let mut failed = 0;
    let mut passed = 0;
    let mut failed_queries = Vec::new();
    
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
            Ok(stmt) => {
                passed += 1;
                },
            Err(x) => {
                failed += 1;
                failed_queries.push(query);
            }
        };
    }
    if failed > 0 {
        for query in failed_queries {
            println!("{}", query);
        }
        panic!("Failed tests: {} of {}", failed, passed + failed);
    }

}
