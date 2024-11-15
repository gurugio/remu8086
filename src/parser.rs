use pest::iterators::Pair;
use pest_derive::Parser;

/*
This derive(Parser) generates Rule implicitly.
*/
#[derive(Parser)]
#[grammar = "assembly.pest"] // grammar file
pub struct AssemblyParser;

// Separate function for unittest
fn _imm_to_num(s: &str) -> Result<u16, String> {
    if s.starts_with("0x") {
        u16::from_str_radix(&s[2..], 16).map_err(|_| "Invalid hex number".to_string())
    } else if s.ends_with('h') {
        u16::from_str_radix(&s[..s.len() - 1], 16).map_err(|_| "Invalid hex number".to_string())
    } else {
        Err("imm is not a valid hex format".to_string())
    }
}

pub fn imm_to_num(s: &Pair<Rule>) -> Result<u16, String> {
    // Three imm forms: 0x1234, 0abcdh, 1abch
    if s.as_rule() != Rule::imm {
        return Err("Tried to parse something else imm".to_string());
    }
    _imm_to_num(s.as_str())
}

pub fn mem_to_num(s: &Pair<Rule>) -> Result<usize, String> {
    // [0x1234], word ptr [0x1234], byte ptr [0x1234] -> 0x1234
    // get number between [ and ]
    let s = s.as_str();
    if let Some(start) = s.find('[') {
        if let Some(end) = s.find(']') {
            if start < end {
                let r: u16 = _imm_to_num(&s[start + 1..end]).unwrap();
                return Ok(r as usize);
            }
        }
    }
    Err("Failed to parse memory address: No valid number found between brackets".to_string())
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use pest::Parser;

    #[test]
    fn test_parser_imm() {
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
    fn test_parser_imm_failure() {
        AssemblyParser::parse(Rule::imm, "1234").expect("Decimal is not allowed");
    }

    #[test]
    fn test_parser_register() {
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
    fn test_parser_operand() {
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
    fn test_parser_instruction_imm() {
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
    fn test_parser_instruction_reg() {
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
    fn test_parser_mem() {
        let instruction = AssemblyParser::parse(Rule::instruction, "mov [0x1234], ax")
            .unwrap()
            .next()
            .unwrap();
        assert_eq!(Rule::mov, instruction.as_rule());
        assert_eq!("mov [0x1234], ax", instruction.as_str());

        let mut inner = instruction.into_inner();
        let mem16 = inner.next().unwrap();
        assert_eq!(Rule::mem16, mem16.as_rule());
        assert_eq!("[0x1234]", mem16.as_str());

        let ax = inner.next().unwrap(); // second operands is imm
        assert_eq!(Rule::reg16, ax.as_rule());
        assert_eq!("ax", ax.as_str());

        let instruction = AssemblyParser::parse(Rule::instruction, "mov ax, word ptr [0x1234]")
            .unwrap()
            .next()
            .unwrap();
        assert_eq!(Rule::mov, instruction.as_rule());
        let mut inner = instruction.into_inner();
        let ax = inner.next().unwrap();
        assert_eq!(Rule::reg16, ax.as_rule());
        assert_eq!("ax", ax.as_str());
        let mem16 = inner.next().unwrap(); // second operands is imm
        assert_eq!(Rule::mem16, mem16.as_rule());
        assert_eq!("word ptr [0x1234]", mem16.as_str());

        assert_eq!(Ok(0x1234), mem_to_num(&mem16));

        let instruction = AssemblyParser::parse(Rule::instruction, "mov al, byte ptr [12h]")
            .unwrap()
            .next()
            .unwrap();
        assert_eq!(Rule::mov, instruction.as_rule());
        assert_eq!("mov al, byte ptr [12h]", instruction.as_str());
        let mut inner = instruction.into_inner();
        let al = inner.next().unwrap();
        assert_eq!(Rule::reg8, al.as_rule());
        assert_eq!("al", al.as_str());
        let mem8 = inner.next().unwrap(); // second operands is imm
        assert_eq!(Rule::mem8, mem8.as_rule());
        assert_eq!("byte ptr [12h]", mem8.as_str());

        assert_eq!(Ok(0x12), mem_to_num(&mem8));
    }

    #[test]
    fn test_parser_program() {
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

    #[test]
    fn test_parser_imm_to_num() {
        assert_eq!(Ok(0x1a3), _imm_to_num("0x1a3"));
        assert_eq!(Ok(0x123), _imm_to_num("123h"));
        assert_eq!(Ok(0xabc), _imm_to_num("0abch"));
        assert_eq!(Ok(0x45), _imm_to_num("045h"));

        assert_eq!(Err("Invalid hex number".to_owned()), _imm_to_num("0xghi"));
    }
}
