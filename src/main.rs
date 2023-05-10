mod ast;
mod parser;

#[macro_use]
extern crate lalrpop_util;

fn main() {
    println!("Hello, world!");
}

#[test]
fn hello_world() {
    // fake test
    main();
}
