use std::io::{BufRead, Read};
use std::collections::HashMap;
use std::cell::RefCell;
// screen word for words as digits
fn screen_str(src:RefCell<String>) {
    let screen_iter = vec![
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
        "zero",
    ];
    let replace_ptrn = HashMap::from([
        ("one","1"),
        ("two","2"),
        ("three","3"),
        ("four","4"),
        ("five","5"),
        ("six","6"),
        ("seven","7"),
        ("eight","8"),
        ("nine","9"),
        ("zero","0"),
    ]);
    // screen the src for each word in screen_iter 
    for word in screen_iter {
        let value = src.borrow_mut();
        src.replace(value.replace(replace_ptrn[word],word));
    }
}
// recursive functions for finding matches
// subject is the text which a match is searched for
fn match_finder(subject:&str) -> String{
    let ret = String::with_capacity(2);
    match_helper(&subject,ret)
}
// ret is a string that is changed recursively(will contain match result at the end of recursive workflow)
fn match_helper(subject:&str,ret:String) -> String {
    if let Some(front) = subject.chars().next() {
        return match front {
            // add char to ret if letter is number
            front if front.is_digit(10) => {
                // shadow ret with a new one
                let mut ret = ret;
                if ret.len() < 2 {
                    ret.push(front)
                } else {
                    // https://doc.rust-lang.org/std/primitive.char.html#method.encode_utf8
                    // current code allocates on heap, from utf8 can use a buffer to convert without allocation.
                    // this line will run various times due to recursion so maybe use a buffer
                    let mut utf8_buff = [0;4];
                    ret.replace_range(1..2,front.encode_utf8(&mut utf8_buff)); // consider char char.encode_utf8 for performance
                }
                match_helper(&subject[1..], ret)
            },
            // skip if char is letter
            _ => {
                match_helper(&subject[1..], ret)
            }
        }
    }
    // check if ret is one value
    // if it is put it again
    // return ret if no characters left in string
    if ret.len() == 1 {
        return ret.repeat(2);
    }
    ret
}
// block can either be big string of text or a file handle
// (any place text can come from)
// remake to use recursion
fn parse_block(block:&mut dyn BufRead) -> i32 {
    // sum of digits
    let mut sum_vec: Vec<i32> = Vec::new();
    //make a char buffer
    let mut buf = String::with_capacity(64); // allocate one big size to minimize frequency of allocations
    let mut _count:usize = 0;  // # of reads for debug purposes
    while let Ok(n) = block.read_line(&mut buf) {
        if n == 0 {
            //print!("{}",count);
            break;
        }
        // handle line matching here
        let digits_str = match_finder(&mut buf);
        println!("Digits:{}",digits_str);
        if digits_str == "" || digits_str == "00" {
            continue;
        } 
        let digits:i32 = digits_str.parse().expect("Error converting value: {digits_str}");
        sum_vec.push(digits);
        buf.clear(); // prepare buffer for next line
        _count+=1;
    }
    // do rest of parsing
    sum_vec.into_iter().reduce(|sum,x| sum + x).expect("Err summing vec...")
}    
fn main() {
    let mut txt = String::new();
    std::fs::File::open("input.txt").unwrap().read_to_string(&mut txt).expect("ERR:could not find input.txt file");
    println!("Sum is {}",parse_block(&mut txt.as_bytes()))
}
