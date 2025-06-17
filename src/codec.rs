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
