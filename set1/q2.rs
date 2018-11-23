// TIL: buffer is just an array of bytes
// Q: is it a reference to, or can it act on?
// A: well, it could be acted on if you passed a mutable reference, like
// &mut {var-name}
// Question: i.e. necessarily &[u8], or can be [u8] ?
// Answer: Lol, I finally read the docs. 
// https://doc.rust-lang.org/book/second-edition/ch04-02-references-and-borrowing.html

fn fixed_xor(buffer1: &[u8], buffer2: &[u8]) {
    // not sure if len is right?
    // also, since buffer - any datatype?
    // so <T> as type?
    // nts: assert makes sure a boolean is true @ run time

    // kk, takes 2 equal length buffers
    assert_eq!(buffer1.len(), buffer2.len()); 

    
    


}

fn main() {
    fixed_xor(&b1, &b2);
}
