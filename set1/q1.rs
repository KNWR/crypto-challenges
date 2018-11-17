fn main(hex_string: String) {
    // 1. convert string into hex
        // instructions say, operate on raw bytes
        // assuming way to represent raw bytes is u8 -- 8 bits in a byte
        // so, need to convert string into u8 into hex
        // to operate on each requires a for loop on the char of the array (the string)
        // &hex_string because it's a slice of the array'   
    for character in &hex_string[0..String::from(&hex_string).len()]

        match character {
            '0'...'9' => (character as u8 - '0' as u8),
            'a'...'f' => 10 + (character as u8 - 'a' as u8)
        }
    // 2. convert hex into bytes?
    // 3. convert bytes into base64
    // 4. output base64 -> string
}

fn character_to_hex(c: char) -> u8 {

}
