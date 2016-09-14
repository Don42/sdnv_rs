extern crate std;

pub struct SDNVDecoder {
    max_length: u32,
}

impl SDNVDecoder {
    pub fn new() -> SDNVDecoder {
        SDNVDecoder {
            max_length: 9,
        }
    }

    pub fn set_max_length(&mut self, max_length: u32) -> &mut Self {
        self.max_length = match max_length {
            0 => 9,
            _ => std::cmp::min(9, max_length),
        };
        self
    }

    pub fn decode(&self, input: &[u8]) -> Option<(u64, u32)> {
        let mut content: u64 = 0;
        let mut count: u32 = 0;
        for c in input {
            count += 1;
            content <<= 7;
            content += (c & 0x7Fu8) as u64;
            if (0x80u8 & c) == 0 {
                return Some((content, count));
            } else if count >= self.max_length {
                return None;
            }
        }
        return None;
    }
}

pub fn decode(input: &[u8]) -> Option<(u64, u32)> {
    SDNVDecoder::new().decode(input)
}
