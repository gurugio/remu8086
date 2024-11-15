use std::cell::RefCell;
use std::fmt;

pub struct Memory {
    data: [u8; 1024 * 1024], // 1MB 크기의 배열
    last_address: RefCell<usize>,
}

impl Memory {
    pub fn boot() -> Self {
        Memory {
            data: [0; 1024 * 1024], // 배열을 0으로 초기화
            last_address: RefCell::new(0),
        }
    }

    // 메모리에서 읽기
    pub fn read8(&self, address: usize) -> u8 {
        *self.last_address.borrow_mut() = address;
        self.data[address]
    }
    pub fn read16(&self, address: usize) -> u16 {
        *self.last_address.borrow_mut() = address;
        self.data[address] as u16 | (self.data[address + 1] as u16) << 8
    }

    // 메모리에 쓰기
    pub fn write8(&mut self, address: usize, value: u8) {
        *self.last_address.borrow_mut() = address;
        self.data[address] = value;
    }

    pub fn write16(&mut self, address: usize, value: u16) {
        *self.last_address.borrow_mut() = address;
        self.data[address] = (value & 0xff) as u8;
        self.data[address + 1] = ((value & 0xff00) >> 8) as u8;
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
        assert_eq!(0xABCD, memory.read16(0));
        assert_eq!(0, *memory.last_address.borrow());
        memory.write16(0, 0xabcd);
        assert_eq!(0xcd, memory.read8(0));
        assert_eq!(0xab, memory.read8(1));
    }
}
