#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub struct VarInt(usize);

impl VarInt {
    pub fn encoded_size(&self) -> u32 {
        match self.0 {
            0...0xFC => 1,
            0xFD...0xFFFF => 3,
            0x10000...0xFFFFFFFF => 5,
            _ => 9,
        }
    }

    pub fn to_u64(self) -> u64 {
        self.0 as u64
    }
}

impl From<u32> for VarInt {
    fn from(num: u32) -> Self {
        VarInt(num as usize)
    }
}

impl From<u64> for VarInt {
    fn from(num: u64) -> Self {
        VarInt(num as usize)
    }
}

impl From<usize> for VarInt {
    fn from(num: usize) -> Self {
        VarInt(num)
    }
}
