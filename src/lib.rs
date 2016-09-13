
fn decode(input: &[u8], max_length: u32) -> Option<(u64, u32)> {
    let mut content: u64 = 0;
    let mut count: u32 = 0;
    let max_length = match max_length {
        0 => 9,
        _ => std::cmp::min(9, max_length),
    };
    for c in input {
        count += 1;
        content <<= 7;
        content += (c & 0x7Fu8) as u64;
        if (0x80u8 & c) == 0 {
            return Some((content, count));
        } else if max_length != 0 && count >= max_length {
            return None;
        }
    }
    return None;
}

#[cfg(test)]
mod tests {
    #[test]
    fn decode_01() {
        let code: [u8; 2] = [0x95, 0x3C];
        let expected = Some((0xABC, 2));
        decode_test(&code, expected);
    }
    #[test]
    fn decode_02() {
        let code: [u8; 1] = [0x7F];
        let expected = Some((0x7F, 1));
        decode_test(&code, expected);
    }
    #[test]
    fn decode_03() {
        let code: [u8; 1] = [0x44];
        let expected = Some((0x44, 1));
        decode_test(&code, expected);
    }
    #[test]
    fn decode_04() {
        let code: [u8; 2] = [0xA4, 0x34];
        let expected = Some((0x1234, 2));
        decode_test(&code, expected);
    }
    #[test]
    fn decode_05() {
        let code: [u8; 3] = [0x81, 0x84, 0x34];
        let expected = Some((0x4234, 3));
        decode_test(&code, expected);
    }
    #[test]
    fn decode_06() {
        let code: [u8; 10] = [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];
        let expected = None;
        decode_test(&code, expected);
    }
    #[test]
    fn decode_07() {
        let code: [u8; 9] = [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];
        let expected = None;
        decode_test(&code, expected);
    }
    #[test]
    fn decode_08() {
        let code: [u8; 9] = [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x7F];
        let expected = Some((0x7FFFFFFFFFFFFFFF, 9));
        decode_test(&code, expected);
    }
    fn decode_test(code: &[u8], expected: Option<(u64, u32)>) {
        let result = super::decode(&code, 0);
        assert_eq!(expected, result)
    }

}
