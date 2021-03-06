
pub mod decoder;
pub mod encoder;

pub use decoder::{SDNVDecoder, decode};
pub use encoder::{SDNVEncoder, encode};



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
        use super::SDNVDecoder;
        let result = SDNVDecoder::new()
            .set_max_length(0)
            .decode(&code);
        assert_eq!(expected, result)
    }

    #[test]
    fn encode_01() {
        let content = 0xABC;
        let expected = Some(vec![0x95, 0x3C]);
        encode_test(content, expected);
    }

    #[test]
    fn encode_02() {
        let content = 0xFFFFFFFFFFFFFFFF;
        let expected = None;
        encode_test(content, expected);
    }

    #[test]
    fn encode_03() {
        let content = 0x7FFFFFFFFFFFFFFF;
        let expected = Some(vec![0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x7F]);
        encode_test(content, expected);
    }

    #[test]
    fn encode_04() {
        let content = 0x7F;
        let expected = Some(vec![0x7F]);
        encode_test(content, expected);
    }
    #[test]
    fn encode_05() {
        let content = 0x44;
        let expected = Some(vec![0x44]);
        encode_test(content, expected);
    }
    #[test]
    fn encode_06() {
        let content = 0x1234;
        let expected = Some(vec![0xA4, 0x34]);
        encode_test(content, expected);
    }
    #[test]
    fn encode_07() {
        let content = 0x4234;
        let expected = Some(vec![0x81, 0x84, 0x34]);
        encode_test(content, expected);
    }

    fn encode_test(content: u64, expected: Option<Vec<u8>>) {
        use super::SDNVEncoder;
        let result = SDNVEncoder::new()
            .encode(content);
        assert_eq!(expected, result)
    }

    #[test]
    fn circle_test_01() {
        use super::{SDNVDecoder, SDNVEncoder};
        let content = 0xbeef;
        let intermediate = SDNVEncoder::new().encode(content);
        let result = SDNVDecoder::new().decode(intermediate.unwrap().as_slice());
        assert_eq!((content, 3), result.unwrap())
    }

    #[test]
    fn circle_test_02() {
        use super::{SDNVDecoder, SDNVEncoder};
        let input = vec![0x81, 0x84, 0x34];
        let intermediate = SDNVDecoder::new().decode(input.as_slice()).unwrap();
        let result = SDNVEncoder::new().encode(intermediate.0).unwrap();
        assert_eq!(input, result);
        assert_eq!(intermediate.1, 3);
    }
}
