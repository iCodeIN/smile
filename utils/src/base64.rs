pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

const BASE64_MAP: [char; 64] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9', '+', '/',
];

// For each 3 bytes we encode 4 base64 characters.
// Output length is always a multiple of 4.
// If input length is not a multiple of 3 then padding is used ('=').
pub fn encode(bytes: &[u8]) -> String {
    let mut res = String::with_capacity(2 * bytes.len());
    let mut i = 0;
    let mut n;

    while i < bytes.len() {
        // First char
        n = bytes[i] >> 2;
        res.push(BASE64_MAP[n as usize]);
        // Second char
        n = (bytes[i] & 0x03) << 4;
        i += 1;
        if i == bytes.len() {
            res.push(BASE64_MAP[n as usize]);
            res.push_str("==");
            break;
        }
        n |= bytes[i] >> 4;
        res.push(BASE64_MAP[n as usize]);
        // Third char
        n = (bytes[i] & 0x0F) << 2;
        i += 1;
        if i == bytes.len() {
            res.push(BASE64_MAP[n as usize]);
            res.push('=');
            break;
        }
        n |= bytes[i] >> 6;
        res.push(BASE64_MAP[n as usize]);
        // Fourth char
        n = bytes[i] & 0x3f;
        res.push(BASE64_MAP[n as usize]);
        i += 1;
    }

    res.shrink_to_fit();
    res
}

const BASE64_UNMAP: [u8; 128] = [
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 62, 255, 255, 255, 63, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 255,
    255, 255, 0, 255, 255, 255, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18,
    19, 20, 21, 22, 23, 24, 25, 255, 255, 255, 255, 255, 255, 26, 27, 28, 29, 30, 31, 32, 33, 34,
    35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 255, 255, 255, 255, 255,
];

pub fn decode(s: &str) -> Result<Vec<u8>> {
    if (s.len() & 0x03) != 0 {
        return Err("Invalid base64 length".into());
    }
    if !s.is_ascii() {
        return Err("Invalid base64 encoding (not ascii)".into());
    }

    let mut i = 0;

    let mut res = Vec::with_capacity(s.len() >> 2 * 3);

    let bytes = s.as_bytes();
    while i < bytes.len() {
        let b0 = bytes[i];
        let b1 = bytes[i + 1];
        let b2 = bytes[i + 2];
        let b3 = bytes[i + 3];

        let n0 = BASE64_UNMAP[b0 as usize];
        let n1 = BASE64_UNMAP[b1 as usize];
        let n2 = BASE64_UNMAP[b2 as usize];
        let n3 = BASE64_UNMAP[b3 as usize];
        if n0 == 255 || n1 == 255 || n2 == 255 || n3 == 255 {
            return Err("Invalid base64 encoding".into());
        }

        res.push(n0 << 2 | n1 >> 4);

        if b2 as char == '=' {
            break;
        }
        res.push(n1 << 4 | n2 >> 2);

        if b3 as char == '=' {
            break;
        }
        res.push(n2 << 6 | n3);

        i += 4;
    }

    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_works() {
        assert_eq!(encode(&[]), "");
        assert_eq!(encode(&[1]), "AQ==");
        assert_eq!(encode(&[1, 2]), "AQI=");
        assert_eq!(encode(&[1, 2, 3]), "AQID");

        let bytes = [
            99, 114, 121, 112, 116, 111, 123, 65, 83, 67, 73, 73, 95, 112, 114, 49, 110, 116, 52,
            98, 108, 51, 125,
        ];
        assert_eq!(encode(&bytes), "Y3J5cHRve0FTQ0lJX3ByMW50NGJsM30=");
    }

    #[test]
    fn decode_works() {
        assert_eq!(decode("").unwrap(), []);
        assert_eq!(decode("AQ==").unwrap(), [1]);
        assert_eq!(decode("AQI=").unwrap(), [1, 2]);
        assert_eq!(decode("AQID").unwrap(), [1, 2, 3]);

        let bytes = [
            99, 114, 121, 112, 116, 111, 123, 65, 83, 67, 73, 73, 95, 112, 114, 49, 110, 116, 52,
            98, 108, 51, 125,
        ];
        assert_eq!(
            decode("Y3J5cHRve0FTQ0lJX3ByMW50NGJsM30=").unwrap(),
            bytes
        );
    }
}
