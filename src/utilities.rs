pub fn word_from_bytes(bytes:[u8; 4])->u32{
    let mut word:u32 = bytes[3] as u32;
    word += (bytes[2] as u32 ) << 8;
    word += (bytes[1] as u32 ) << 16;
    word += (bytes[0] as u32 ) << 24;

    word
}

pub fn bytes_from_word(word:u32)->[u8;4]{
    let mut bytes:[u8;4] = [0;4];
    bytes[3] = (word & 0xFF ) as u8;
    bytes[2] = ((word & 0xFF00) >> 8 ) as u8;
    bytes[1] = ((word & 0xFF0000) >> 16 ) as u8;
    bytes[0] = ((word & 0xFF000000) >> 24 ) as u8;

    bytes
}



//Tests
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_word_from_bytes() {
        assert_eq!(0x85342591, word_from_bytes( [0x85, 0x34, 0x25, 0x91]) );
        assert_eq!(0xaece3212, word_from_bytes( [0xae, 0xce, 0x32, 0x12]) );
        assert_eq!(0xb5000474, word_from_bytes( [0xb5, 0x00, 0x04, 0x74]) );
    }

    #[test]
    fn test_bytes_from_word() {
        assert_eq!([0x85, 0x34, 0x25, 0x91], bytes_from_word( 0x85342591 ) );
        assert_eq!([0xae, 0xce, 0x32, 0x12], bytes_from_word( 0xaece3212 ) );
        assert_eq!([0xb5, 0x00, 0x04, 0x74], bytes_from_word( 0xb5000474 ) );
    }
}
