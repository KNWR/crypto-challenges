// use std::io;


fn hex_to_base64(hex_string: String) -> String {
    // 1. convert string into hex, represented as bytes
        // instructions say, operate on raw bytes
        // assuming way to represent raw bytes is u8 -- 8 bits in a byte
            // so, need to convert string into u8 that represents the hex
        // each hex char is 4 bits 
        // to operate on each character, can use a for loop on the char of the array (the string)
        // &hex_string because it's a slice of the array   

    // let mut bytes: &[u8]; // no, because these are fixed length ... could be fine, just allocate length of string 
    // let mut v: Vec<u8> = Vec::new();

    // for character in &hex_string {
    //     v.push(character.as_bytes()); 
    // }

    // convert string into byte vector 
    // https://doc.rust-lang.org/std/string/struct.String.html#method.into_bytes
    let bytes = hex_string.into_bytes()

    // 2. convert bytes into base64
        // each base64 char is 6 bits 
        // for every 3 bytes, we have four 6 bit base64 characters (LCM(6,4) == 24)




    // 3. output base64 -> string
}

fn main(hex_string: String, base64_string: String) {

    is_correct = assert_eq!(hex_to_base_64(hex_string), base64_string);
    if is_correct { print!("It works! Succesfully ...") } else { print!("Oops! Not quite there yet – think through it ...") };

}
