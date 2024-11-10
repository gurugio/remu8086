/*
paste macro works like the token concatenation(# and ##) of C language.
e.g. [<caller_ $mod>] => caller_mov
*/

#[macro_export]
macro_rules! define_caller_one {
    ($mod:ident, $cpu:ident) => {
        paste! {
            fn [<caller_ $mod>]($cpu: &mut CpuContext, operands: &mut Pairs<Rule>) {
                let first_operand = operands.next().unwrap();
                $mod::[<handler_ $mod>]($cpu, first_operand);
            }
        }
    };
}

#[macro_export]
macro_rules! define_handler_one {
    ($mod:ident, $first:ident, $cpu:ident, $body:block) => {
        paste! {
            pub fn [<handler_ $mod>]($cpu: &mut CpuContext, $first: Pair<Rule>) {
                $body
            }
        }
    };
}

#[macro_export]
macro_rules! define_caller_two {
    ($mod:ident, $cpu:ident) => {
        paste! {
            fn [<caller_ $mod>]($cpu: &mut CpuContext, operands: &mut Pairs<Rule>) {
                let first_operand = operands.next().unwrap();
                let second_operand = operands.next().unwrap();
                $mod::[<handler_ $mod>]($cpu, first_operand, second_operand);
            }
        }
    };
}

#[macro_export]
macro_rules! define_handler_two {
    ($mod:ident, $first:ident, $second:ident, $cpu:ident, $body:block) => {
        paste! {
            pub fn [<handler_ $mod>]($cpu: &mut CpuContext, $first: Pair<Rule>, $second: Pair<Rule>) {
                $body
            }
        }
    };
}
