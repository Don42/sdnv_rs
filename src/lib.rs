
fn decode(input: &[u8], max_length: u32) -> Option<u64> {
    let mut content: u64 = 0;
    let mut count: u32 = 0;
    for c in input {
        count += 1;
        content <<= 7;
        content += (c & 0x7Fu8) as u64;
        if (0x80u8 & c) == 0 {
            return Some(content);
        }
    }
    return None;
}

#[cfg(test)]
mod tests {
    #[test]
    fn decode_01() {
        let code: [u8; 2] = [0x95, 0x3C];
        let expected: u64 = 0xABC;
        decode_test(&code, expected);
    }
    #[test]
    fn decode_02() {
        let code: [u8; 1] = [0x7F];
        let expected: u64 = 0x7F;
        decode_test(&code, expected);
    }
    #[test]
    fn decode_03() {
        let code: [u8; 1] = [0x44];
        let expected: u64 = 0x44;
        decode_test(&code, expected);
    }
    #[test]
    fn decode_04() {
        let code: [u8; 2] = [0xA4, 0x34];
        let expected: u64 = 0x1234;
        decode_test(&code, expected);
    }
    #[test]
    fn decode_05() {
        let code: [u8; 3] = [0x81, 0x84, 0x34];
        let expected: u64 = 0x4234;
        decode_test(&code, expected);
    }
    fn decode_test(code: &[u8], expected: u64) {
        let result = super::decode(&code, 0);
        assert_eq!(expected, result.unwrap())
    }

}
