use crate::{Hash, Uint256, VarInt};
use encodings::{FromHex, FromHexError, ToHex};
use std::fmt;
use std::ops;

#[derive(thiserror::Error, Debug)]
pub enum BufferError {
    #[error("Read Out of Bounds")]
    OutOfBounds,
    #[error(transparent)]
    InvalidString(#[from] std::string::FromUtf8Error),
    #[error("Non-Minimal VarInt")]
    NonMinimalVarInt,
}

pub type Result<T> = std::result::Result<T, BufferError>;

//Our version of Buffer that is implemented in bio - > https://github.com/bcoin-org/bufio
#[derive(Default, PartialEq, Clone, Eq)]
pub struct Buffer {
    data: Vec<u8>,
    offset: usize,
}

impl Buffer {
    pub fn new() -> Self {
        Buffer::default()
    }

    //Accept anything that implements into buffer.
    // pub fn new_with() -> Self {

    // }

    //Unsigned Integers - Little Endian
    pub fn write_u8(&mut self, data: u8) {
        self.data.extend_from_slice(&data.to_le_bytes());
    }

    pub fn write_u16(&mut self, data: u16) {
        self.data.extend_from_slice(&data.to_le_bytes());
    }

    pub fn write_u32(&mut self, data: u32) {
        self.data.extend_from_slice(&data.to_le_bytes());
    }

    pub fn write_u64(&mut self, data: u64) {
        self.data.extend_from_slice(&data.to_le_bytes());
    }

    //TODO u128

    pub fn write_u256(&mut self, data: Uint256) {
        self.data.extend_from_slice(&data.to_le_bytes());
    }

    //Big Endian
    pub fn write_u8_be(&mut self, data: u8) {
        self.data.extend_from_slice(&data.to_be_bytes());
    }

    pub fn write_u16_be(&mut self, data: u16) {
        self.data.extend_from_slice(&data.to_be_bytes());
    }

    pub fn write_u32_be(&mut self, data: u32) {
        self.data.extend_from_slice(&data.to_be_bytes());
    }

    pub fn write_u64_be(&mut self, data: u64) {
        self.data.extend_from_slice(&data.to_be_bytes());
    }

    //TODO u128, and u256

    //Signed Integers
    pub fn write_i8(&mut self, data: i8) {
        self.data.extend_from_slice(&data.to_le_bytes());
    }

    pub fn write_i16(&mut self, data: i16) {
        self.data.extend_from_slice(&data.to_le_bytes());
    }

    pub fn write_i32(&mut self, data: i32) {
        self.data.extend_from_slice(&data.to_le_bytes());
    }

    pub fn write_i64(&mut self, data: i64) {
        self.data.extend_from_slice(&data.to_le_bytes());
    }

    //Big Endian
    pub fn write_i8_be(&mut self, data: i8) {
        self.data.extend_from_slice(&data.to_be_bytes());
    }

    pub fn write_i16_be(&mut self, data: i16) {
        self.data.extend_from_slice(&data.to_be_bytes());
    }

    pub fn write_i32_be(&mut self, data: i32) {
        self.data.extend_from_slice(&data.to_be_bytes());
    }

    pub fn write_i64_be(&mut self, data: i64) {
        self.data.extend_from_slice(&data.to_be_bytes());
    }

    pub fn write_bytes(&mut self, bytes: &[u8]) {
        //TODO should we clone here or just pass in
        self.data.extend_from_slice(bytes);
    }

    pub fn write_var_bytes(&mut self, bytes: &[u8]) {
        self.write_varint(bytes.len());

        if bytes.is_empty() {
            return;
        }

        self.data.extend_from_slice(bytes);
    }

    pub fn write_str(&mut self, string: &str) {
        self.data.extend_from_slice(string.as_bytes());
    }

    pub fn write_string(&mut self, string: String) {
        self.data.extend_from_slice(string.as_bytes());
    }

    pub fn write_hash(&mut self, hash: Hash) {
        self.data.extend(hash.to_array());
    }

    pub fn write_varint(&mut self, data: usize) {
        if data < 0xFD {
            self.write_u8(data as u8);
            return;
        }

        if data < 0xFFFF {
            self.write_u8(0xFD);
            self.write_u16(data as u16);
            return;
        }

        if data < 0xFFFFFFFF {
            self.write_u8(0xFE);
            self.write_u32(data as u32);
            return;
        }

        self.write_u8(0xFF);
        self.write_u64(data as u64);
    }

    pub fn fill(&mut self, value: u8, amount: usize) {
        //See what's faster, this or resize_with/resize TODO
        let fill_amount = vec![value; amount];
        self.data.extend(fill_amount);
    }

    pub fn extend(&mut self, buffer: Buffer) {
        self.data.extend_from_slice(&buffer);
    }

    pub fn extend_from_slice(&mut self, slice: &[u8]) {
        self.data.extend_from_slice(slice);
    }

    ////Return Hex string of the buffer, Consumes the Buffer
    pub fn into_hex(self) -> String {
        self.data.to_hex()
    }

    //Check for length
    pub fn check(&self, size: usize) -> Result<()> {
        if self.offset + size > self.data.len() {
            return Err(BufferError::OutOfBounds);
        }
        Ok(())
    }

    //These can probably all be macro'd out.
    pub fn read_u8(&mut self) -> Result<u8> {
        self.check(1)?;
        let result = self.data[self.offset];

        self.offset += 1;

        Ok(result)
    }

    pub fn read_u16(&mut self) -> Result<u16> {
        self.check(2)?;
        let range = self.offset..self.offset + 2;

        let mut buf = [0; 2];
        buf.copy_from_slice(&self.data[range]);

        let ret = u16::from_le_bytes(buf);

        self.offset += 2;

        Ok(ret)
    }

    pub fn read_u16_be(&mut self) -> Result<u16> {
        self.check(2)?;
        let range = self.offset..self.offset + 2;

        let mut buf = [0; 2];
        buf.copy_from_slice(&self.data[range]);

        let ret = u16::from_be_bytes(buf);

        self.offset += 2;

        Ok(ret)
    }

    //TODO do we see any need for reading u24s, 48s, etc?

    pub fn read_u32(&mut self) -> Result<u32> {
        self.check(4)?;
        let range = self.offset..self.offset + 4;

        let mut buf = [0; 4];
        buf.copy_from_slice(&self.data[range]);

        let ret = u32::from_le_bytes(buf);

        self.offset += 4;

        Ok(ret)
    }

    pub fn read_u32_be(&mut self) -> Result<u32> {
        self.check(4)?;
        let range = self.offset..self.offset + 4;

        let mut buf = [0; 4];
        buf.copy_from_slice(&self.data[range]);

        let ret = u32::from_be_bytes(buf);

        self.offset += 4;

        Ok(ret)
    }

    pub fn read_u64(&mut self) -> Result<u64> {
        self.check(8)?;
        let range = self.offset..self.offset + 8;

        let mut buf = [0; 8];
        buf.copy_from_slice(&self.data[range]);

        let ret = u64::from_le_bytes(buf);

        self.offset += 8;

        Ok(ret)
    }

    pub fn read_u64_be(&mut self) -> Result<u64> {
        self.check(8)?;
        let range = self.offset..self.offset + 8;

        let mut buf = [0; 8];
        buf.copy_from_slice(&self.data[range]);

        let ret = u64::from_be_bytes(buf);

        self.offset += 8;

        Ok(ret)
    }

    pub fn read_u256(&mut self) -> Result<Uint256> {
        self.check(32)?;

        let range = self.offset..self.offset + 32;
        let ret = Uint256::from_bytes(&self.data[range]);

        self.offset += 32;

        Ok(ret)
    }

    pub fn read_varint(&mut self) -> Result<VarInt> {
        let len = self.read_u8()?;

        match len {
            0xFF => {
                let num = self.read_u64()?;
                if num < 0x100000000 {
                    Err(BufferError::NonMinimalVarInt)
                } else {
                    Ok(VarInt::from(num))
                }
            }
            0xFE => {
                let num = self.read_u32()?;
                if num < 0x10000 {
                    Err(BufferError::NonMinimalVarInt)
                } else {
                    Ok(VarInt::from(num))
                }
            }
            0xFD => {
                let num = self.read_u16()?;
                if num < 0xFD {
                    Err(BufferError::NonMinimalVarInt)
                } else {
                    Ok(VarInt::from(num))
                }
            }

            len => Ok(VarInt::from(len)),
        }
    }

    pub fn read_string(&mut self, size: usize) -> Result<String> {
        self.check(size)?;

        let range = self.offset..self.offset + size;
        let ret = String::from_utf8(self.data[range].to_vec())?;

        self.offset += size;

        Ok(ret)
    }

    pub fn read_bytes(&mut self, size: usize) -> Result<Vec<u8>> {
        self.check(size)?;

        let range = self.offset..self.offset + size;
        let ret = self.data[range].to_vec();

        self.offset += size;

        Ok(ret)
    }

    pub fn read_var_bytes(&mut self) -> Result<Vec<u8>> {
        let length = self.read_varint()?;

        //TODO make an as_usize for varint.
        let size = length.as_u64() as usize;

        let range = self.offset..self.offset + size;
        let ret = self.data[range].to_vec();

        self.offset += size;

        Ok(ret)
    }

    pub fn read_hash(&mut self) -> Result<Hash> {
        self.check(32)?;

        let mut array = [0; 32];

        let range = self.offset..self.offset + 32;
        array.copy_from_slice(&self.data[range]);

        let hash = Hash::from(array);

        self.offset += 32;

        Ok(hash)
    }

    //Essentially shifts the offset to offset += off
    pub fn seek(&mut self, off: usize) -> Result<()> {
        self.check(off)?;

        self.offset += off;

        Ok(())
    }

    pub fn to_bytes(&self) -> &[u8] {
        &self.data
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.data.clone()
    }
}

impl From<Vec<u8>> for Buffer {
    fn from(buf: Vec<u8>) -> Self {
        Buffer {
            data: buf,
            offset: 0,
        }
    }
}

impl From<&[u8]> for Buffer {
    fn from(buf: &[u8]) -> Self {
        Buffer {
            data: buf.to_vec(),
            offset: 0,
        }
    }
}

//@todo commenting out for now. Not sure how we should implement this. FromStr technically should
//return an error, but writing a string to a Buffer will never error out. Also it might add some
//confusion as to whether we are creating the buffer from the raw string or from a hex string. So
//it might just be in our interest to not have these funcitons period.
//TODO review, seems inefficent
//impl From<&str> for Buffer {
//    fn from(buf: &str) -> Self {
//        Buffer {
//            data: buf.as_bytes().to_vec(),
//            offset: 0,
//        }
//    }
//}

////TODO review, seems inefficent
//impl From<String> for Buffer {
//    fn from(buf: String) -> Self {
//        Buffer {
//            data: buf.as_bytes().to_vec(),
//            offset: 0,
//        }
//    }
//}

//Allows us to grab specific bytes from the buffer e.g.
//grab the merkle tree from the middle of the buffer.
impl ops::Deref for Buffer {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

//Allows us to grab specific bytes from the buffer e.g.
//grab the merkle tree from the middle of the buffer.
//Same as above, but allows us to grab those bytes and mutable, thus changing them without
//having to allocate more mem.
impl ops::DerefMut for Buffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

//Allows Buffer to be used as a reference for a [u8] TODO double check this.
//And thoroughly comment for everyone
impl AsRef<[u8]> for Buffer {
    fn as_ref(&self) -> &[u8] {
        &self.data
    }
}

//Allows Buffer to be used as a mut for a [u8] TODO double check this.
//And thoroughly comment for everyone
impl AsMut<[u8]> for Buffer {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }
}

impl FromHex for Buffer {
    type Error = FromHexError;

    fn from_hex<T: AsRef<[u8]>>(hex: T) -> std::result::Result<Self, Self::Error> {
        Ok(Buffer::from(Vec::from_hex(hex)?))
    }
}

impl ToHex for Buffer {
    fn to_hex(&self) -> String {
        self.data.to_hex()
    }
}

impl fmt::Display for Buffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Offset: {}, Buffer: {})",
            self.offset,
            self.data.to_hex(),
        )
    }
}

impl fmt::Debug for Buffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Offset: {}, Buffer: {})",
            self.offset,
            self.data.to_hex(),
        )
    }
}

#[cfg(feature = "serialization")]
impl serde::Serialize for Buffer {
    fn serialize<S: serde::Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        if s.is_human_readable() {
            s.serialize_str(&self.to_hex())
        } else {
            s.serialize_bytes(&self[..])
        }
    }
}

#[cfg(feature = "serialization")]
impl<'de> serde::Deserialize<'de> for Buffer {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Buffer, D::Error> {
        if d.is_human_readable() {
            struct HexVisitor;

            impl<'de> serde::de::Visitor<'de> for HexVisitor {
                type Value = Buffer;

                fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    formatter.write_str("an ASCII hex string")
                }

                fn visit_bytes<E>(self, v: &[u8]) -> std::result::Result<Self::Value, E>
                where
                    E: ::serde::de::Error,
                {
                    if let Ok(hex) = ::std::str::from_utf8(v) {
                        Buffer::from_hex(hex).map_err(E::custom)
                    } else {
                        return Err(E::invalid_value(serde::de::Unexpected::Bytes(v), &self));
                    }
                }

                fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
                where
                    E: ::serde::de::Error,
                {
                    Buffer::from_hex(v).map_err(E::custom)
                }
            }

            d.deserialize_str(HexVisitor)
        } else {
            struct BytesVisitor;

            impl<'de> ::serde::de::Visitor<'de> for BytesVisitor {
                type Value = Buffer;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str("a bytestring")
                }

                fn visit_bytes<E>(self, v: &[u8]) -> std::result::Result<Self::Value, E>
                where
                    E: ::serde::de::Error,
                {
                    Ok(Buffer::from(v))
                }
            }

            d.deserialize_bytes(BytesVisitor)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_u32() {
        let version: u32 = 123456789;

        let mut buffer = Buffer::new();

        buffer.write_u32(version);

        assert_eq!(buffer, Buffer::from([21, 205, 91, 7].to_vec()));
    }

    // #[test]
    // fn test_write_hash() {
    //     let hash = Hash::from("bb42edce1895f9a969e81d7371ec113a0966e5d55035a84f87ca098e4f0a1a86");

    //     let mut buffer = Buffer::new();

    //     buffer.write_hash(hash);
    // }

    #[test]
    fn test_to_hex() {
        let version: u32 = 123456789;

        let mut buffer = Buffer::new();

        buffer.write_u32(version);

        assert_eq!(buffer, Buffer::from([21, 205, 91, 7].to_vec()));

        let hex = buffer.to_hex();

        assert_eq!(hex, "15cd5b07")
    }

    #[test]
    fn test_into_hex() {
        let version: u32 = 123456789;

        let mut buffer = Buffer::new();

        buffer.write_u32(version);

        assert_eq!(buffer, Buffer::from([21, 205, 91, 7].to_vec()));

        let hex = buffer.into_hex();

        assert_eq!(hex, "15cd5b07")
    }

    #[test]
    fn test_from_hex() {
        //@todo would really like to support this type of from/toHx.
        let buffer = Buffer::from_hex("FF00").unwrap();

        dbg!(&buffer);

        assert_eq!(buffer, Buffer::from(vec![255, 0]));
    }

    #[cfg(feature = "serialization")]
    #[test]
    fn test_serde() {
        use serde_test::{assert_tokens, Configure, Token};

        let version: u32 = 123456789;

        let mut buffer = Buffer::new();

        buffer.write_u32(version);

        static version_bytes: [u8; 4] = [21, 205, 91, 7];

        let buffer_readable = buffer.clone();

        assert_tokens(&buffer.compact(), &[Token::BorrowedBytes(&version_bytes)]);
        assert_tokens(&buffer_readable.readable(), &[Token::Str("15cd5b07")]);
    }
}
