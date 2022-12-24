mod add;
mod cpucontext;

fn main() {
    //let line = "mov ax, 1";
    println!("Hello, world!");
    // read a line and call instruction handler
    let mut test_context = cpucontext::CpuContext::default();

    // TODO: make a parser to parse a line into Instruction
    // mov ax,1
    let test_inst = cpucontext::Instruction {
        operation: "mov".to_string(),
        left_operand: Some(cpucontext::Operand {
            field: "ax".to_string(),
            flag: cpucontext::OperFlag::Reg16,
        }),
        right_operand: Some(cpucontext::Operand {
            field: "1".to_string(),
            flag: cpucontext::OperFlag::Imm16,
        }),
    };

    add::do_add(&mut test_context, &test_inst);
    println!("{:?}", test_context);
}
