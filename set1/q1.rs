// use std::io;
// use std::iter::Iterator;
// use std::collections::BitVec;

fn hex_to_base64(hex_string: String) -> String {
    /* 1. convert string into hex, represented as bytes */

    // convert hex string into byte vector 
    // https://doc.rust-lang.org/std/string/struct.String.html#method.into_bytes
    let bytes = hex_string.into_bytes();

    /* 2. convert bytes into base64 */

    // get our vector of bytes as bits now
    // https://doc.rust-lang.org/1.2.0/std/collections/struct.BitVec.html#method.from_bytes
    let bv = BitVec::from_bytes(&bytes);

    // create the string output ahead of time, will be building it throughout
    let mut result = String::new();

    // each base64 char is 6 bits, each hex char is 4 bits
    // for every 6 bytes, we have four 6 bit base64 characters (LCM(6,4) == 24) 

    // this pattern below - a loop, and within it, a tuple of .next() iterator objects to collect values
    // and then matching against those tuple items
    // is from https://github.com/jakerr/cryptopals-rust/blob/master/src/conversions.rs 
    // it's interesting - clumsy given the need for 'break's and matching all, but interesting way to
    // get to MVP soln by explicitly enumerating the cases in a match statement
    // then, can come back later or as writing the expressions corresponding to each case
    // and notice the patterns, then rewriting the match condition, etc., finding a new structure
    // for the algo
    loop {
        set = ( bv.next(), bv.next(),  )

        
    }



    // 3. output base64 -> string
}

fn main(hex_string: String, base64_string: String) {

    is_correct = assert_eq!(hex_to_base_64(hex_string), base64_string);
    if is_correct { print!("It works! Succesfully ...") } else { print!("Oops! Not quite there yet – think through it ...") };

}


// excellent, succinct explanation of the task: https://www.quora.com/How-does-base64-encoding-work/answer/Ke-Pi 
// the answer above it gives a decent walk through of actually doing the conversion by hand
// which is worthwhile