use pest::iterators::Pairs;
use pest::Parser;
use pest_derive::Parser;
use std::fs;

#[derive(Parser)]
#[grammar = "assembly.pest"] // grammar file
struct AssemblyParser;

fn parse_mov(operands: &mut Pairs<'_, Rule>) {
    println!("parse_mov");

    // Below prints each operands as "Operand" but we should know the type of operand: register, number or label.
    let first_operand = operands.next().unwrap();
    println!(
        "first: rule={:?} text={}",
        first_operand,
        first_operand.as_str()
    );
    let second_operand = operands.next().unwrap();
    println!(
        "second: rule={:?} text={}",
        second_operand,
        second_operand.as_str()
    );

    let sec = AssemblyParser::parse(Rule::operand, second_operand.as_str()).unwrap();
    println!("sec={:?}", sec);
}

fn main() {
    /*let pairs = AssemblyParser::parse(Rule::instruction, "mov ax, bx")
    .expect("Failed to parse")
    .next()
    .unwrap();*/

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
                parse_mov(&mut inner_rule);
            }
            _ => println!("else:{}", line),
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_number() {
        /*
        Rule::number is a silent rule. Pair has only Rule::hex or Rule::digit.
        Then we can parse hex or digit into hex-number or digit-number because we know what it is.
        */
        let hex = AssemblyParser::parse(Rule::number, "0x1234")
            .unwrap()
            .next()
            .unwrap();
        assert_eq!(Rule::hex, hex.as_rule());
        assert_eq!("0x1234", hex.as_str());

        let digit = AssemblyParser::parse(Rule::number, "1234")
            .unwrap()
            .next()
            .unwrap();
        assert_eq!(Rule::digit, digit.as_rule());
        assert_eq!("1234", digit.as_str());
    }

    #[test]
    fn test_register() {
        let reg = AssemblyParser::parse(Rule::register, "ax")
            .unwrap()
            .next()
            .unwrap();
        assert_eq!(Rule::register, reg.as_rule());
        assert_eq!("ax", reg.as_str());

        // The Rule::register is the terminal syntax. So there is no inner rules.
        let inner = reg.into_inner().next();
        assert_eq!(None, inner);
    }

    #[test]
    fn test_operand() {
        let operand = AssemblyParser::parse(Rule::operand, "ax")
            .unwrap()
            .next()
            .unwrap();
        assert_eq!(Rule::operand, operand.as_rule());
        assert_eq!("ax", operand.as_str());

        // Rule::operand includes register and number.
        let inner = operand.into_inner().next().unwrap();
        assert_eq!(Rule::register, inner.as_rule());
        assert_eq!("ax", inner.as_str());

        let operand = AssemblyParser::parse(Rule::operand, "0x1234")
            .unwrap()
            .next()
            .unwrap();
        assert_eq!(Rule::operand, operand.as_rule());
        assert_eq!("0x1234", operand.as_str());

        // Rule::operand includes register and number.
        let inner = operand.into_inner().next().unwrap();
        assert_eq!(Rule::hex, inner.as_rule());
        assert_eq!("0x1234", inner.as_str());
    }

    #[test]
    fn test_instruction() {
        let instruction = AssemblyParser::parse(Rule::instruction, "mov ax, bx")
            .unwrap()
            .next()
            .unwrap();
        println!(
            "instruction: rule={:?} text={}",
            instruction.as_rule(), // mov
            instruction.as_str()   // mov ax, bx
        );
        let mul = AssemblyParser::parse(Rule::mul, "mul ax").unwrap();
        println!("mul:{}", mul.as_str());

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
    }
}
