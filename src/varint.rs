#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub struct VarInt(u64);

impl VarInt {
    pub fn encoded_size(&self) -> u32 {
        match self.0 {
            0..=0xFC => 1,
            0xFD..=0xFFFF => 3,
            0x10000..=0xFFFFFFFF => 5,
            _ => 9,
        }
    }

    pub fn to_u64(self) -> u64 {
        self.0
    }

    pub fn as_u64(&self) -> u64 {
        self.0
    }
}

impl From<usize> for VarInt {
    fn from(num: usize) -> Self {
        VarInt(num as u64)
    }
}

impl From<u8> for VarInt {
    fn from(num: u8) -> Self {
        VarInt(num as u64)
    }
}

impl From<u16> for VarInt {
    fn from(num: u16) -> Self {
        VarInt(num as u64)
    }
}

impl From<u32> for VarInt {
    fn from(num: u32) -> Self {
        VarInt(num as u64)
    }
}

impl From<u64> for VarInt {
    fn from(num: u64) -> Self {
        VarInt(num)
    }
}
