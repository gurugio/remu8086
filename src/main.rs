mod cpucontext;

use pest::Parser;
use pest_derive::Parser;
use std::collections::HashMap;
use std::fs;

#[derive(Parser)]
#[grammar = "assembly.pest"] // grammar file
struct AssemblyParser;

fn main() {
    /*let pairs = AssemblyParser::parse(Rule::instruction, "mov ax, bx")
    .expect("Failed to parse")
    .next()
    .unwrap();*/
    let num = AssemblyParser::parse(Rule::number, "0x1234");
    println!("{:?}", num);
    println!("text:{}", num.unwrap().as_str());

    let reg = AssemblyParser::parse(Rule::register, "ax").unwrap();
    println!("reg:{}", reg.as_str());

    let operand = AssemblyParser::parse(Rule::operand, "ax").unwrap();
    println!("operand:{}", operand.as_str());

    let mul = AssemblyParser::parse(Rule::mul, "mul ax").unwrap();
    println!("mul:{}", mul.as_str());

    let instruction = AssemblyParser::parse(Rule::instruction, "mov ax, bx")
        .unwrap()
        .next()
        .unwrap();
    println!(
        "instruction: rule={:?} text={}",
        instruction.as_rule(), // mov
        instruction.as_str()   // mov ax, bx
    );

    /*
    instruction type is Pair. The into_inner method returns Pairs that is an iterator on Pair of enclosed rules
    mov = { "mov" + operand + ',' + operand } => Enclosed rules are operand and operand.
    So below for loop returns
    Rule: operand Text: ax
    Rule: operand Text: bx
     */
    for pair in instruction.into_inner() {
        println!("operand: Rule: {:?} Text={}", pair.as_rule(), pair.as_str());
    }

    println!("########### try to read a file");
    let unparsed_file = fs::read_to_string("example.as").expect("cannot read file");
    let file = AssemblyParser::parse(Rule::program, &unparsed_file)
        .expect("unsuccessful parse") // unwrap the parse result
        .next()
        .unwrap(); // get and unwrap the `file` rule; never fails
    for line in file.into_inner() {
        match line.as_rule() {
            Rule::mov => {
                println!("mov:{:?}", line);
                let mut inner_rule = line.into_inner();

                // Below prints each operands as "Operand" but we should know the type of operand: register, number or label.
                let first_operand = inner_rule.next().unwrap();
                println!(
                    "first: rule={:?} text={}",
                    first_operand,
                    first_operand.as_str()
                );
                let second_operand = inner_rule.next().unwrap();
                println!(
                    "second: rule={:?} text={}",
                    second_operand,
                    second_operand.as_str()
                );

                let sec = AssemblyParser::parse(Rule::operand, second_operand.as_str()).unwrap();
                println!("sec={:?}", sec);
            }
            _ => println!("else:{}", line),
        }
    }
}
