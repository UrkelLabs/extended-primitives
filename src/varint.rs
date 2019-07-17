#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub struct VarInt(u64);

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
        self.0
    }

    pub fn as_u64(&self) -> u64 {
        self.0
    }
}

impl<T> From<T> for VarInt
where
    T: Into<u64>,
{
    fn from(num: T) -> Self {
        VarInt(num.into())
    }
}
