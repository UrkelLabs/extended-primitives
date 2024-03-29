use encodings::hex::{FromHex, FromHexError, ToHex};
use std::fmt;
use std::str::FromStr;

#[derive(Copy, Clone, PartialEq, Eq, Default, PartialOrd, Ord, Hash)]
//Possibly make this generic on the size?
//Not sure if we'll need that, but just a reminder
//I think we might actually want to implement this as a trait?
//@todo from<Uint256>
pub struct Hash([u8; 32]);

impl Hash {
    pub fn to_array(&self) -> [u8; 32] {
        self.0
    }

    pub fn is_null(&self) -> bool {
        for byte in self.0.iter() {
            if *byte != 0 {
                return false;
            }
        }
        true
    }
}

//Needs to be TryFrom
//Need more checks here for length, and errors
impl From<Vec<u8>> for Hash {
    fn from(hex_vec: Vec<u8>) -> Self {
        let mut array = [0; 32];
        array.copy_from_slice(&hex_vec);
        Hash(array)
    }
}

//This should only be implemented on Blake2b hash
//Redo this when we split to blake2b/ run into problems TODO
impl From<[u8; 32]> for Hash {
    fn from(bytes: [u8; 32]) -> Self {
        Hash(bytes)
    }
}

impl AsRef<[u8]> for Hash {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl FromHex for Hash {
    type Error = FromHexError;
    fn from_hex<T: AsRef<[u8]>>(hex: T) -> std::result::Result<Self, Self::Error> {
        let bytes = Vec::from_hex(hex)?;
        if bytes.len() != 32 {
            Err(FromHexError::InvalidHexLength)
        } else {
            let mut ret = [0; 32];
            ret.copy_from_slice(&bytes);
            Ok(Hash::from(ret))
        }
    }
}

impl ToHex for Hash {
    fn to_hex(&self) -> String {
        self.0.to_vec().to_hex()
    }
}

impl FromStr for Hash {
    type Err = FromHexError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Hash::from_hex(s)
    }
}

impl fmt::Display for Hash {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}

impl fmt::Debug for Hash {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}

#[cfg(feature = "serialization")]
impl serde::Serialize for Hash {
    fn serialize<S: serde::Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        if s.is_human_readable() {
            s.serialize_str(&self.to_hex())
        } else {
            s.serialize_bytes(&self.to_array())
        }
    }
}

#[cfg(feature = "serialization")]
impl<'de> serde::Deserialize<'de> for Hash {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Hash, D::Error> {
        if d.is_human_readable() {
            struct HexVisitor;

            impl<'de> serde::de::Visitor<'de> for HexVisitor {
                type Value = Hash;

                fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    formatter.write_str("an ASCII hex string")
                }

                fn visit_bytes<E>(self, v: &[u8]) -> std::result::Result<Self::Value, E>
                where
                    E: ::serde::de::Error,
                {
                    if let Ok(hex) = ::std::str::from_utf8(v) {
                        Hash::from_hex(hex).map_err(E::custom)
                    } else {
                        return Err(E::invalid_value(serde::de::Unexpected::Bytes(v), &self));
                    }
                }

                fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
                where
                    E: ::serde::de::Error,
                {
                    Hash::from_hex(v).map_err(E::custom)
                }
            }

            d.deserialize_str(HexVisitor)
        } else {
            struct BytesVisitor;

            impl<'de> ::serde::de::Visitor<'de> for BytesVisitor {
                type Value = Hash;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str("a bytestring")
                }

                fn visit_bytes<E>(self, v: &[u8]) -> std::result::Result<Self::Value, E>
                where
                    E: ::serde::de::Error,
                {
                    if v.len() != 32 {
                        Err(E::invalid_length(v.len(), &stringify!(32)))
                    } else {
                        let mut ret = [0; 32];
                        ret.copy_from_slice(v);
                        Ok(Hash(ret))
                    }
                }
            }

            d.deserialize_bytes(BytesVisitor)
        }
    }
}

//TODO need to test this, and add testing for serde stuff.
