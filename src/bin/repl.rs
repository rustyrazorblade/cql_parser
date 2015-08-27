extern crate cql_parser;

use std::io;
use std::io::Write;

use cql_parser::cql::parse;


fn main() {
    println!("REPL");
    let mut reader = io::stdin();
    let mut writer = io::stdout();

    let mut buf = String::new();


    loop {
        print!("> ");
        writer.flush();

        reader.read_line(&mut buf);
        println!("Buffer: {}", buf);

        let parsed = parse(&buf);
    }
}
