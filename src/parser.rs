use pest_derive::Parser;

/*
This derive(Parser) generates Rule implicitly.
*/
#[derive(Parser)]
#[grammar = "assembly.pest"] // grammar file
pub struct AssemblyParser;

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use pest::Parser;

    #[test]
    fn test_imm() {
        let hex = AssemblyParser::parse(Rule::imm, "0x1234")
            .unwrap()
            .next()
            .unwrap();
        assert_eq!(Rule::imm, hex.as_rule());
        assert_eq!("0x1234", hex.as_str());

        let hex = AssemblyParser::parse(Rule::imm, "0abcdh")
            .unwrap()
            .next()
            .unwrap();
        assert_eq!(Rule::imm, hex.as_rule());
        assert_eq!("0abcdh", hex.as_str());

        let hex = AssemblyParser::parse(Rule::imm, "1abch")
            .unwrap()
            .next()
            .unwrap();
        assert_eq!(Rule::imm, hex.as_rule());
        assert_eq!("1abch", hex.as_str());
    }

    #[test]
    #[should_panic(expected = "Decimal is not allowed")]
    fn test_imm_failure() {
        AssemblyParser::parse(Rule::imm, "1234").expect("Decimal is not allowed");
    }

    #[test]
    fn test_register() {
        let reg = AssemblyParser::parse(Rule::register, "ax")
            .unwrap()
            .next()
            .unwrap();
        assert_eq!(Rule::reg16, reg.as_rule());
        assert_eq!("ax", reg.as_str());

        let reg = AssemblyParser::parse(Rule::register, "al")
            .unwrap()
            .next()
            .unwrap();
        assert_eq!(Rule::reg8, reg.as_rule());
        assert_eq!("al", reg.as_str());

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
        assert_eq!(Rule::reg16, operand.as_rule());
        assert_eq!("ax", operand.as_str());

        // Rule::operand includes register and imm.
        //let inner = operand.into_inner().next().unwrap();
        //assert_eq!(Rule::register, inner.as_rule());
        //assert_eq!("ax", inner.as_str());

        let operand = AssemblyParser::parse(Rule::operand, "0x1234")
            .unwrap()
            .next()
            .unwrap();
        assert_eq!(Rule::imm, operand.as_rule());
        assert_eq!("0x1234", operand.as_str());

        // Rule::operand includes register and imm.
        //let inner = operand.into_inner().next().unwrap();
        //assert_eq!(Rule::hex, inner.as_rule());
        //assert_eq!("0x1234", inner.as_str());
    }

    #[test]
    fn test_instruction_imm() {
        let instruction = AssemblyParser::parse(Rule::instruction, "mov ax, 0x100")
            .unwrap()
            .next()
            .unwrap();
        assert_eq!(Rule::mov, instruction.as_rule());
        assert_eq!("mov ax, 0x100", instruction.as_str());

        let mut inner = instruction.into_inner();
        let ax = inner.next().unwrap();
        assert_eq!(Rule::reg16, ax.as_rule());
        assert_eq!("ax", ax.as_str());

        let imm = inner.next().unwrap(); // second operands is imm
        assert_eq!(Rule::imm, imm.as_rule());
        assert_eq!("0x100", imm.as_str());

        let instruction = AssemblyParser::parse(Rule::instruction, "mov al, 0x12")
            .unwrap()
            .next()
            .unwrap();
        assert_eq!(Rule::mov, instruction.as_rule());
        assert_eq!("mov al, 0x12", instruction.as_str());

        let mut inner = instruction.into_inner();
        let al = inner.next().unwrap();
        assert_eq!(Rule::reg8, al.as_rule());
        assert_eq!("al", al.as_str());

        let imm = inner.next().unwrap(); // second operands is imm
        assert_eq!(Rule::imm, imm.as_rule());
        assert_eq!("0x12", imm.as_str());
    }

    #[test]
    fn test_instruction_reg() {
        let instruction = AssemblyParser::parse(Rule::instruction, "mov ax, bx")
            .unwrap()
            .next()
            .unwrap();
        assert_eq!(Rule::mov, instruction.as_rule());
        assert_eq!("mov ax, bx", instruction.as_str());

        let mut inner = instruction.into_inner();
        let ax = inner.next().unwrap();
        assert_eq!(Rule::reg16, ax.as_rule());
        assert_eq!("ax", ax.as_str());

        let bx = inner.next().unwrap(); // second operands is imm
        assert_eq!(Rule::reg16, bx.as_rule());
        assert_eq!("bx", bx.as_str());

        // another instruction with a single operand
        let mul = AssemblyParser::parse(Rule::mul, "mul ax")
            .unwrap()
            .next()
            .unwrap();
        assert_eq!(Rule::mul, mul.as_rule());
        assert_eq!("mul ax", mul.as_str());
        let mut inner = mul.into_inner();
        let ax = inner.next().unwrap();
        assert_eq!(Rule::reg16, ax.as_rule());
        assert_eq!("ax", ax.as_str());
    }

    #[test]
    fn test_mem() {
        let instruction = AssemblyParser::parse(Rule::instruction, "mov [0x1234], ax")
            .unwrap()
            .next()
            .unwrap();
        assert_eq!(Rule::mov, instruction.as_rule());
        assert_eq!("mov [0x1234], ax", instruction.as_str());

        let mut inner = instruction.into_inner();
        let memx = inner.next().unwrap();
        assert_eq!(Rule::memx, memx.as_rule());
        assert_eq!("[0x1234]", memx.as_str());

        let ax = inner.next().unwrap(); // second operands is imm
        assert_eq!(Rule::reg16, ax.as_rule());
        assert_eq!("ax", ax.as_str());

        let instruction = AssemblyParser::parse(Rule::instruction, "mov ax, word ptr [0x1234]")
            .unwrap()
            .next()
            .unwrap();
        assert_eq!(Rule::mov, instruction.as_rule());
        assert_eq!("mov ax, word ptr [0x1234]", instruction.as_str());

        let instruction = AssemblyParser::parse(Rule::instruction, "mov al, byte ptr [0x12]")
            .unwrap()
            .next()
            .unwrap();
        assert_eq!(Rule::mov, instruction.as_rule());
        assert_eq!("mov al, byte ptr [0x12]", instruction.as_str());
    }

    #[test]
    fn test_program() {
        let program = "mov ax, bx\nadd cx, 0x1234\njmp 1\n1:";
        let file = AssemblyParser::parse(Rule::program, program)
            .expect("Failed to parse a file with Rule::program rule") // unwrap the parse result
            .next()
            .unwrap(); // get and unwrap the `file` rule; never fails
        let mut lines = file.into_inner();
        let mov = lines.next().unwrap();
        assert_eq!(Rule::mov, mov.as_rule());
        let add = lines.next().unwrap();
        assert_eq!(Rule::add, add.as_rule());
        let jmp = lines.next().unwrap();
        assert_eq!(Rule::jmp, jmp.as_rule());
    }
}
