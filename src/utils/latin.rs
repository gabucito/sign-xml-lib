use encoding_rs::WINDOWS_1252;

pub fn to_latin1(input: &str) -> Vec<u8> {
    let mut buffer = Vec::with_capacity(input.len());
    for ch in input.chars() {
        if ch.is_ascii() {
            // Directly push ASCII characters
            buffer.push(ch as u8);
        } else {
            // Convert non-ASCII characters to a placeholder if they are not in the Latin-1 range
            buffer.push(utf8_to_latin1(ch).unwrap_or(b'?'));
        }
    }
    buffer
}

// This function converts a Unicode character to its Latin-1 equivalent, if possible
fn utf8_to_latin1(ch: char) -> Option<u8> {
    if ch as u32 <= 0xFF {
        Some(ch as u8)
    } else {
        None // Return None for characters outside the Latin-1 range
    }
}

pub fn convert_to_iso_8859_1(input: &str) -> Result<Vec<u8>, &'static str> {
    let (cow, _encoding_used, errors) = WINDOWS_1252.encode(input);
    if errors {
        Err("Encoding conversion error")
    } else {
        Ok(cow.into_owned())
    }
}
