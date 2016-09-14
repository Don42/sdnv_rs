extern crate std;

pub struct SDNVEncoder {
}

impl SDNVEncoder {
    pub fn new() -> SDNVEncoder {
        SDNVEncoder { }
    }

    pub fn encode(&self, content: u64) -> Option<Vec<u8>> {
        let mut content = content;
        let mut flag = 0;
        let mut ret_array: Vec<u8> = Vec::new();

        loop {
            let new_bits: u8 = (content & 0x7F) as u8 + flag;
            content >>= 7;
            ret_array.push(new_bits);

            if flag == 0 {
                flag = 0x80;
            }

            if ret_array.len() > 9 {
                return None;
            }

            if content == 0 {
                ret_array.reverse();
                return Some(ret_array);
            }

        }
    }
}

pub fn encode(content: u64) -> Option<Vec<u8>> {
    SDNVEncoder::new().encode(content)
}
