// use std::io;
// use std::iter::Iterator;
use std::collections::BitVec;

fn hex_to_base64(hex_string: String) -> String {
    /* 1. convert string into hex, represented as bytes */

    // convert hex string into byte vector 
    // https://doc.rust-lang.org/std/string/struct.String.html#method.into_bytes
    let bytes = hex_string.into_bytes();

    /* 2. convert bytes into base64 */

    // create the output ahead of time, will be building it throughout
    let mut result = Vec::new();

    // each base64 char is 6 bits, each hex char is 4 bits
    // for every 6 bytes, we have four 6 bit base64 characters (LCM(6,4) == 24) 

    // why not just programmatically create this with a range?
    // This is faster, more quickly accessed: https://stackoverflow.com/a/19061862 
    // Using a match statement with a _ => fail! statement likely more secure / 
    // cleaner code, see https://github.com/garrettr/cryptopals/blob/master/1-1-base64.rs 
    // above has some beautiful bit manipulation, really great
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

    //not at all clear to me why base64 needs to exist in blocks of 24 bits,
    // beyond byte compatibility? looks like it needs to be in blocks of 24 bits
    // which helps a lot to know ...

    // TODO: padding to have blocks of 24 bits

    // TODO: putting from bytes to ascii via the table


    // 3. output base64 -> string
    print!("{}", &result);

    result
}

fn main(hex_string: String, base64_string: String) {

    is_correct = assert_eq!(hex_to_base_64(hex_string), base64_string);
    if is_correct { 
        print!("It works! Succesfully ...")
    } else { 
        print!("Oops! Not quite there yet – think through it ...")
    };

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