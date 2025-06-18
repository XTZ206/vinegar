fn encode_keyword(databytes: &[u8], keybytes: &[u8]) -> Vec<u8> {
    let mut buffer: Vec<u8> = Vec::new();
    for (idx, &databyte) in databytes.iter().enumerate() {
        let keybyte = keybytes[idx as usize % keybytes.len()];
        let encoded = databyte ^ keybyte;
        buffer.push(encoded);
    }
    return buffer;
}

fn decode_keyword(databytes: &[u8], keybytes: &[u8]) -> Vec<u8> {
    let mut buffer: Vec<u8> = Vec::new();
    for (idx, &databyte) in databytes.iter().enumerate() {
        let keybyte = keybytes[idx as usize % keybytes.len()];
        let decoded = databyte ^ keybyte;
        buffer.push(decoded);
    }
    return buffer;
}

fn encode_freq(databytes: &[u8]) -> Vec<u8> {
    let mut buffer: Vec<u8> = Vec::new();
    let mut counts: [u8; 256] = [0; 256];

    for &databyte in databytes {
        counts[databyte as usize] += 1;
    }

    for &databyte in databytes {
        let freq = counts[databyte as usize];
        let encoded = databyte ^ freq;
        buffer.push(encoded);
    }

    return buffer;
}

fn decode_freq(databytes: &[u8]) -> Vec<u8> {
    let mut buffer: Vec<u8> = Vec::new();
    let mut counts: [u8; 256] = [0; 256];

    for &databyte in databytes {
        counts[databyte as usize] += 1;
    }

    for &databyte in databytes {
        let freq = counts[databyte as usize];
        let decoded = databyte ^ freq;
        buffer.push(decoded);
    }

    return buffer;
}

pub fn encode(bytes: &[u8]) -> Vec<u8> {
    return encode_keyword(bytes, b"VINEGAR");
}

pub fn decode(bytes: &[u8]) -> Vec<u8> {
    return decode_keyword(bytes, b"VINEGAR");
}

pub fn display(bytes: &[u8]) -> String {
    let mut ret = String::new();
    for (idx, &byte) in bytes.iter().enumerate() {
        if idx % 4 == 0 {
            ret.push_str(&format!("{:02x}: ", idx as u8));
        }
        ret.push_str(&format!("{:02x} ", byte));
        if idx % 4 == 3 || idx == bytes.len() - 1 {
            ret.push('\n');
        }
        if idx > 0x3f {
            ret.push_str("...\n");
            break;
        }
    }
    return ret;
}
