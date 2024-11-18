/*
paste macro works like the token concatenation(# and ##) of C language.
e.g. [<caller_ $mod>] => caller_mov
*/

#[macro_export]
macro_rules! caller_one {
    ($mod:ident, $cpu:expr, $memory:expr, $pairs:ident) => {
        paste! {
            let mut inner_rule = $pairs.into_inner();
            let first_operand = inner_rule.next().unwrap();
            $mod::[<handler_ $mod>](&mut $cpu, &mut $memory, first_operand);
        }
    };
}

#[macro_export]
macro_rules! define_handler_one {
    ($mod:ident, $first:ident, $cpu:ident, $memory:ident, $body:block) => {
        paste! {
            pub fn [<handler_ $mod>]($cpu: &mut CpuContext, $memory: &mut Memory, $first: Pair<Rule>) {
                $body
            }
        }
    };
}

#[macro_export]
macro_rules! caller_two {
    ($mod:ident, $cpu:expr, $memory:expr, $pairs:ident) => {
        paste! {
            let mut inner_rule = $pairs.into_inner();
            let first_operand = inner_rule.next().unwrap();
            let second_operand = inner_rule.next().unwrap();
            $mod::[<handler_ $mod>](&mut $cpu, &mut $memory, first_operand, second_operand);
        }
    };
}

#[macro_export]
macro_rules! define_handler_two {
    ($mod:ident, $first:ident, $second:ident, $cpu:ident, $memory:ident, $body:block) => {
        paste! {
            pub fn [<handler_ $mod>]($cpu: &mut CpuContext, $memory: &mut Memory, $first: Pair<Rule>, $second: Pair<Rule>) {
                $body
            }
        }
    };
}

pub fn count_bit(v: u16) -> i32 {
    let mut c = 0;
    let mut v = v;

    while v != 0 {
        if v & 0x1 != 0 {
            c += 1;
        }
        v >>= 1;
    }

    c
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_count_bit() {
        let v = vec![
            0, 1, 2, 3, 4, 5, 6, 7, 0x1000, 0x2000, 0x3000, 0x4000, 0x5000, 0x6000, 0x7000, 0xffff,
        ];
        let r = vec![0, 1, 1, 2, 1, 2, 2, 3, 1, 1, 2, 1, 2, 2, 3, 16];

        for i in 0..v.len() {
            assert_eq!(r[i], count_bit(v[i]));
        }
    }
}
