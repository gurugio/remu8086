#[macro_export]
macro_rules! define_caller_one {
    ($mod:ident, $func:ident, $cpu:ident) => {
        fn $func($cpu: &mut CpuContext, operands: &mut Pairs<Rule>) {
            let first_operand = operands.next().unwrap();
            $mod::$func($cpu, first_operand);
        }
    };
}

#[macro_export]
macro_rules! define_handler_one {
    ($func:ident, $first:ident, $cpu:ident, $body:block) => {
        pub fn $func($cpu: &mut CpuContext, $first: Pair<Rule>) {
            $body
        }
    };
}

#[macro_export]
macro_rules! define_caller_two {
    ($mod:ident, $func:ident, $cpu:ident) => {
        fn $func($cpu: &mut CpuContext, operands: &mut Pairs<Rule>) {
            let first_operand = operands.next().unwrap();
            let second_operand = operands.next().unwrap();
            $mod::$func($cpu, first_operand, second_operand);
        }
    };
}

#[macro_export]
macro_rules! define_handler_two {
    ($func:ident, $first:ident, $second:ident, $cpu:ident, $body:block) => {
        pub fn $func($cpu: &mut CpuContext, $first: Pair<Rule>, $second: Pair<Rule>) {
            $body
        }
    };
}
