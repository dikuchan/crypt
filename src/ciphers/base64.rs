fn into_base(b: u8) -> char {
    let result = match b {
        0x00..=0x19 => b + 0x41,
        0x1A..=0x33 => b + 0x61 - 0x1A,
        0x34..=0x3D => b + 0x30 - 0x34,
        0x3E => 0x2B,
        0x3F => 0x2F,
        _ => unreachable!()
    };

    result as char
}

fn from_base(b: u8) -> u32 {
    let result = match b {
        0x41..=0x5A => b - 0x41,
        0x61..=0x7A => b - 0x61 + 0x1A,
        0x30..=0x39 => b - 0x30 + 0x34,
        0x2B => 0x3E,
        0x2F => 0x3F,
        0x3D => 0x00,
        _ => unreachable!()
    };

    result as u32
}

pub fn encode(string: &str) -> String {
    let mut result = String::new();

    for chunk in string.as_bytes().chunks(3) {
        let shifts: [usize; 3] = [0x10, 0x8, 0x0];
        let d: usize = chunk.iter()
            .zip(shifts.iter())
            .map(|(&x, &y)| (x as usize) << y)
            .sum();

        for b in 0x00..0x04 {
            if chunk.len() >= b {
                let offset = (0x03 - b) * 0x06;
                let shifted = (d >> offset & 0x3F) as u8;
                result.push(into_base(shifted));
            } else {
                result.push('=');
            }
        }
    }

    result
}

pub fn decode(string: &str) -> String {
    let mut result = String::new();

    for chunk in string.as_bytes().chunks(4) {
        let shifts: [u32; 4] = [0x12, 0x0C, 0x06, 0x00];
        let d: u32 = chunk.iter()
            .zip(shifts.iter())
            .map(|(&x, &y)| from_base(x) << y)
            .sum();

        for b in 0x01..0x04 {
            if chunk[b] != 0x3D {
                let offset = (0x03 - b) * 0x08;
                let shifted = std::char::from_u32(d >> offset & 0xFF).unwrap();
                result.push(shifted);
            }
        }
    }

    result
}

#[test]
fn test_base64_encoding() {
    assert_eq!(encode("Hello, world!"), String::from("SGVsbG8sIHdvcmxkIQ=="));
    assert_eq!(encode("Attack AT DAWN"), String::from("QXR0YWNrIEFUIERBV04="));
    assert_eq!(encode("Русские вперед"), String::from("0KDRg9GB0YHQutC40LUg0LLQv9C10YDQtdC0"));
    assert_eq!(encode("こんばんは"), String::from("44GT44KT44Gw44KT44Gv"))
}

#[test]
fn test_base64_decoding() {
    assert_eq!(decode(&encode("Hello, world!")), String::from("Hello, world!"));
    assert_eq!(decode(&encode("Attack AT DAWN")), String::from("Attack AT DAWN"));
}