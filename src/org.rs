use crate::define_handler_one;
use crate::parser::Rule;
use pest::iterators::Pair;

/*
org 100h
*/

define_handler_one!(handle_org, first, {
    match first.as_rule() {
        Rule::imm => {
            println!("handle org {}", first.as_str());
        }
        _ => println!("Not supported operand for org:{:?}", first),
    }
});
