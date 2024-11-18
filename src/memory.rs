use std::cell::RefCell;
use std::fmt;

pub struct Memory {
    data: Box<[u8; 1024 * 1024]>, // 1MB 크기의 배열
    last_address: RefCell<usize>,
}

impl Memory {
    pub fn boot() -> Self {
        // Memory MUST be allocated with Box.
        // Memory structure is an 1MB size array.
        // It generates stack-overflow if it is allocated on the stack.
        Memory {
            data: Box::new([0; 1024*1024]), // 배열을 0으로 초기화
            last_address: RefCell::new(0),
        }
    }

    //
    // BUGBUG!! Add address range check and return error
    //

    // 메모리에서 읽기
    pub fn read8(&self, address: usize) -> u8 {
        *self.last_address.borrow_mut() = address;
        self.data[address]
    }
    pub fn read16(&self, address: usize) -> u16 {
        *self.last_address.borrow_mut() = address;
        // Little-endian: read first address and the lower byte
        self.data[address] as u16 | (self.data[address + 1] as u16) << 8
    }

    // 메모리에 쓰기
    pub fn write8(&mut self, address: usize, value: u8) {
        *self.last_address.borrow_mut() = address;
        self.data[address] = value;
        println!("{:?}", self);
    }

    pub fn write16(&mut self, address: usize, value: u16) {
        // Little-endian: write lower byte first
        *self.last_address.borrow_mut() = address;
        self.data[address] = (value & 0xff) as u8;
        self.data[address + 1] = ((value & 0xff00) >> 8) as u8;
        println!("{:?}", self);
    }
}

// println! memory values around the last accessed address
impl fmt::Debug for Memory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();

        let start = *self.last_address.borrow() & 0xffff0;
        let end = start + 0xf;
        s.push_str(&format!("{:05X}", start));
        s.push(' ');
        for i in start..=end {
            // DO NOT USE read/write method because it changes last_address value
            let d = self.data[i];
            let ss = format!("{:02X}", d);
            s.push_str(&ss);
            if i != end {
                s.push(' ');
            }
        }

        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_read8_write8() {
        let mut memory = Memory::boot();
        memory.write8(0, 0xAB);
        assert_eq!(0, *memory.last_address.borrow());
        memory.write8(1, 0xCD);
        assert_eq!(1, *memory.last_address.borrow());
        assert_eq!(0xAB, memory.read8(0));
        assert_eq!(0, *memory.last_address.borrow());
        assert_eq!(0xCD, memory.read8(1));
        assert_eq!(1, *memory.last_address.borrow());
    }

    #[test]
    fn test_memory_read16_write16() {
        let mut memory = Memory::boot();
        memory.write8(0, 0xCD);
        assert_eq!(0, *memory.last_address.borrow());
        memory.write8(1, 0xAB);
        assert_eq!(1, *memory.last_address.borrow());
        // Check the little-endian reading
        assert_eq!(0xABCD, memory.read16(0));
        assert_eq!(0, *memory.last_address.borrow());
        memory.write16(0, 0xabcd);
        // Check the little-endian writing
        assert_eq!(0xcd, memory.read8(0));
        assert_eq!(0xab, memory.read8(1));
    }

    #[test]
    fn test_memory_debug() {
        let mut memory = Memory::boot();
        memory.write16(0, 0xabcd);
        let s = format!("{:?}", memory);
        assert_eq!("00000 CD AB 00 00 00 00 00 00 00 00 00 00 00 00 00 00", s);
    }
}
