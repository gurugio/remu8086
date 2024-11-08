#[macro_export]
macro_rules! call_handler_one {
    ($mod:ident, $func:ident) => {
        fn $func(operands: &mut Pairs<Rule>) {
            let first_operand = operands.next().unwrap();
            $mod::$func(first_operand);
        }
    };
}

#[macro_export]
macro_rules! define_handler_one {
    ($func:ident, $first:ident, $body:block) => {
        pub fn $func($first: Pair<Rule>) {
            $body
        }
    };
}

#[macro_export]
macro_rules! call_handler_two {
    ($mod:ident, $func:ident) => {
        fn $func(operands: &mut Pairs<Rule>) {
            let first_operand = operands.next().unwrap();
            let second_operand = operands.next().unwrap();
            $mod::$func(first_operand, second_operand);
        }
    };
}

#[macro_export]
macro_rules! define_handler_two {
    ($func:ident, $first:ident, $second:ident, $body:block) => {
        pub fn $func($first: Pair<Rule>, $second: Pair<Rule>) {
            $body
        }
    };
}
