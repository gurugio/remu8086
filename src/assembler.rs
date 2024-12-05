pub fn register_table(reg: &str) -> Result<u8, String> {
    match reg {
        "ax" | "al" => Ok(0),
        "cx" | "cl" => Ok(1),
        "dx" | "dl" => Ok(2),
        "bx" | "bl" => Ok(3),
        "sp" | "ah" => Ok(4),
        "bp" | "ch" => Ok(5),
        "si" | "dh" => Ok(6),
        "di" | "bh" => Ok(7),
        _ => Err(format!("{} is not in the register_table", reg)),
    }
}

pub fn base_index_table(base: Option<&str>, index: Option<&str>) -> Result<u8, String> {
    match (base, index) {
        (Some("bx"), Some("si")) => Ok(0),
        (Some("bx"), Some("di")) => Ok(1),
        (Some("bp"), Some("si")) => Ok(2),
        (Some("bp"), Some("fi")) => Ok(3),
        (None, /* */ Some("si")) => Ok(4),
        (None, /* */ Some("di")) => Ok(5),
        (Some("bp"), None) => Ok(6),
        (Some("bx"), None) => Ok(7),
        _ => Err(format!(
            "{:?} {:?} is not in the base index table",
            base, index
        )),
    }
}
