use std::{fmt::Display, str::FromStr};

use crate::{Error, Result};

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ChunkType {
    bytes: [u8; 4],
}

// http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html#Chunk-naming-conventions

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        self.bytes
    }

    fn is_valid(&self) -> bool {
        let bytes = self.bytes();
        for b in bytes {
            if !b.is_ascii_alphabetic() {
                return false;
            }
        }
        if bytes[2].is_ascii_lowercase() {
            return false;
        }
        true
    }

    fn is_critical(&self) -> bool {
        if self.bytes()[0].is_ascii_uppercase() {
            return true;
        }
        false
    }

    fn is_public(&self) -> bool {
        if self.bytes()[1].is_ascii_uppercase() {
            return true;
        }
        false
    }

    fn is_reserved_bit_valid(&self) -> bool {
        if self.bytes()[2].is_ascii_uppercase() {
            return true;
        }
        false
    }
    fn is_safe_to_copy(&self) -> bool {
        if self.bytes()[3].is_ascii_uppercase() {
            return false;
        }
        true
    }
}

impl FromStr for ChunkType {
    fn from_str(str: &str) -> Result<ChunkType> {
        let bytes: [u8; 4] = str.as_bytes().try_into()?;
        let mut vaild = true;
        for b in bytes {
            if !b.is_ascii_alphabetic() {
                vaild = false;
                break;
            }
        }
        if vaild {
            return Result::Ok(ChunkType { bytes });
        }
        Err("invalid chunk type!")?
    }

    type Err = Error;
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(self.bytes().as_ref()))
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Error;

    fn try_from(bytes: [u8; 4]) -> Result<ChunkType> {
        let mut vaild = true;
        for b in bytes {
            if !b.is_ascii_alphabetic() {
                vaild = false;
                break;
            }
        }
        if vaild {
            return Result::Ok(ChunkType { bytes });
        }
        Err("invalid chunk type!")?
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
