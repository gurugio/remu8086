mod cpucontext;

use pest::Parser;
use pest_derive::Parser;

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
        instruction.as_rule(),
        instruction.as_str()
    );

    /*
    Rule: operand
    Text: ax
    Rule: operand
    Text: bx
     */
    for pair in instruction.into_inner() {
        println!("operand: Rule: {:?} Text={}", pair.as_rule(), pair.as_str());
    }

    // TODO: NEXT THING: how to make a loop to parse one program?
}
