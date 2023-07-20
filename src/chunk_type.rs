use std::{str::FromStr, fmt};
use crate::{Result, Error};
use anyhow::anyhow;

#[derive(Debug, PartialEq, Eq)]
pub struct ChunkType {
    value: [u8; 4]
}

impl TryFrom<[u8;4]> for ChunkType {
    type Error = Error;

    fn try_from(value: [u8;4]) -> Result<Self> {
        if !ChunkType::is_bytes_all_ascii(value) {
            Err(anyhow!("The bytes are not between 65-90 or 97-122, i.e. ASCII"))
        } else {
            Ok(ChunkType {
                value
            })
        }
    }
}

impl FromStr for ChunkType {
    type Err = Error;
    
    fn from_str(s: &str) -> Result<Self> {
        let bytes: [u8;4] = s.as_bytes().try_into().unwrap();
        ChunkType::try_from(bytes)
    }
}

impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = String::from_utf8(self.value.to_vec()).unwrap();
        write!(f, "{}", s)
    }
}

impl ChunkType {
    fn is_bytes_all_ascii(value: [u8;4]) -> bool {
        value.iter().all(|&x| (65<=x&&x<=90) || (97<=x&&x<=122))
    }

    fn is_bytes_reserved_bit_valid(value: [u8;4]) -> bool {
        let third_byte = value[2];
        let bit_5 = third_byte & (1 << 5);
        bit_5 == 0
    }

    pub fn bytes(&self) -> [u8; 4] {
        self.value
    }

    fn is_valid(&self) -> bool {
        ChunkType::is_bytes_all_ascii(self.value) && ChunkType::is_bytes_reserved_bit_valid(self.value)
    }

    fn is_critical(&self) -> bool {
        let first_byte = self.value[0];
        let bit_5 = first_byte & (1 << 5);
        bit_5 == 0
    }

    fn is_public(&self) -> bool {
        let second_byte = self.value[1];
        let bit_5 = second_byte & (1 << 5);
        bit_5 == 0
    }

    fn is_reserved_bit_valid(&self) -> bool {
        ChunkType::is_bytes_reserved_bit_valid(self.value)
    }

    fn is_safe_to_copy(&self) -> bool {
        let fourth_byte = self.value[3];
        let bit_5 = fourth_byte & (1 << 5);
        bit_5 != 0
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
