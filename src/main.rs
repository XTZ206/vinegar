use std::io::Read;

mod codec;

fn main() {
    let mut buffer = Vec::new();
    std::io::stdin()
        .read_to_end(&mut buffer)
        .expect("Failed to read from stdin");
    let encoded = codec::encode(&buffer);

    println!("Encoded Data: ");
    for (idx, &code) in encoded.iter().enumerate() {
        if idx % 4 == 0 {
            print!("{:02x}: ", idx as u8);
        }
        print!("{:02x} ", code);
        if idx % 4 == 3 || idx == encoded.len() - 1 {
            println!();
        }
    }

    let decoded = codec::decode(&encoded);
    println!("Decoded Data: ");
    for (idx, &code) in decoded.iter().enumerate() {
        if idx % 4 == 0 {
            print!("{:02x}: ", idx as u8);
        }
        print!("{:02x} ", code);
        if idx % 4 == 3 || idx == decoded.len() - 1 {
            println!();
        }
    }

    if let Ok(text) = String::from_utf8(decoded) {
        println!("Decoded Text: {}", text);
    } else {
        println!("Decoded data is not valid UTF-8.");
    }
}
