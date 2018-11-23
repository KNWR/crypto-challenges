// use std::io;
// use std::iter::Iterator;
// use std::collections::BitVec;

// abject shock when cs 2208 comes in handy

fn hex_to_base64(hex_string: String) -> String {
    /* 1. convert string into hex, represented as bytes */

    // convert hex string into byte vector 
    // https://doc.rust-lang.org/std/string/struct.String.html#method.into_bytes
    let _bytes = string_to_hex(&hex_string);

    /* 2. convert bytes into base64 */

    // create the output ahead of time, will be building it throughout
    // let result = Vec::new();

    // each base64 char is 6 bits, each hex char is 4 bits
    // for every 6 bytes, we have four 6 bit base64 characters (LCM(6,4) == 24) 

    // why not just programmatically create this with a range?
    // This is faster, more quickly accessed: https://stackoverflow.com/a/19061862 
    // Using a match statement with a _ => fail! statement likely more secure / 
    // cleaner code, see https://github.com/garrettr/cryptopals/blob/master/1-1-base64.rs 
    // above has some beautiful bit manipulation, really great
    // love these diagrams for understanding encoding https://www.imperva.com/blog/wp-content/uploads/sites/9/2018/04/B64-6.png love these

    const BASE_64: [char; 64] = [
        'A','B','C','D','E','F','G','H',
        'I','J','K','L','M','N','O','P',
        'Q','R','S','T','U','V','W','X',
        'Y','Z','a','b','c','d','e','f',
        'g','h','i','j','k','l','m','n',
        'o','p','q','r','s','t','u','v',
        'w','x','y','z','0','1','2','3',
        '4','5','6','7','8','9','+','/'
    ];

    // create vector to return 
    base64_vec = Vec::new();

    // not at all clear to me why base64 needs to exist in blocks of 24 bits since b64's read 6bits @a time
    // beyond byte compatibility? looks like it needs to be in blocks of 24 bits
    // which helps a lot to know ...

    // TODO: padding to have blocks of 24 bits
    for 


    // TODO: putting from bytes to ascii via the table
    base64_vec.push( BASE_64[ **SOMENUMBER** ] );

    // 3. output base64 -> string
    // result;
    // format!("{}", base64_vec as String)
    base64_vec as String;
}

// borrowing @jakerr 's method
// casting, how does as work? https://stackoverflow.com/questions/48795329/what-is-the-difference-between-fromfrom-and-as-in-rust 
// re 'as', see also -- docs -- https://doc.rust-lang.org/book/2018-edition/appendix-01-keywords.html?highlight=casting#keywords-currently-in-use

// each hex digit represents 4 binary digits => half a byte; since we're operating on bytes, we combine them 2 at a time

fn string_to_hex(string: &str) -> Vec<u8> {
    let mut byte_vec = Vec::new();

    let mut cs = string.chars(); 
    // assert! that cs only contains 0...9, a...f
    for _ in 0 ... string.len()/2 {
        // c = chars.next();
        // match c {
        //     '0'...'9' => c as u8,
        //     'a'...'f' => c as u8
        byte_vec.push( chars.next() as u8 | chars.next() as u8 << 4 );
    }

    byte_vec
}


fn main() {
    // even though hex string is consumed, doesn't need to be mutable?
    let hex_string : String = String::from("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
    let base64_string : String = String::from("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");

    if hex_to_base64(hex_string) == base64_string { 
        print!("It works! Succesfully ...");
    } else { 
        print!("Oops! Not quite there yet – think through it ...");
    }
}


// excellent, succinct explanation of the task: https://www.quora.com/How-does-base64-encoding-work/answer/Ke-Pi 
// the answer above it gives a decent walk through of actually doing the conversion by hand
// which is worthwhile

// brilliant, in ruby https://medium.com/@christ.blais/whats-the-deal-with-base64-c93263b92dd6

// note to self: note the pattern from https://github.com/jakerr/cryptopals-rust/blob/master/src/conversions.rs 
// - a loop, and within it, a tuple of .next() iterator objects to collect values and 
// then matching against those tuple items. interesting - clumsy given the need for 
// 'break's and matching all, but interesting way to get to MVP soln by explicitly 
// enumerating the cases in a match statement then, can come back later or as 
// writing the expressions corresponding to each case and notice the patterns, then 
// rewriting the match condition, etc., finding a new structure for the algo
