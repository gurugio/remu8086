/*
paste macro works like the token concatenation(# and ##) of C language.
e.g. [<caller_ $mod>] => caller_mov
*/

#[macro_export]
macro_rules! caller_one {
    ($mod:ident, $cpu:ident, $pairs:ident) => {
        paste! {
            let mut inner_rule = $pairs.into_inner();
            let first_operand = inner_rule.next().unwrap();
            $mod::[<handler_ $mod>](&mut $cpu, first_operand);
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
macro_rules! caller_two {
    ($mod:ident, $cpu:ident, $pairs:ident) => {
        paste! {
            let mut inner_rule = $pairs.into_inner();
            let first_operand = inner_rule.next().unwrap();
            let second_operand = inner_rule.next().unwrap();
            $mod::[<handler_ $mod>](&mut $cpu, first_operand, second_operand);
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
