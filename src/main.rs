use std::io::{BufRead, Read};
use std::collections::HashMap;
use priority_queue::DoublePriorityQueue; 

// screen word for words as digits
fn screen_str(src:&str) -> String {
    let mut res = String::with_capacity(src.len());
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
    // note that words aren't burnt one translated
    // ex: eighttwo -> 82 not 8wo
    let replace_ptrn = HashMap::from([
        ("one","o1e"),
        ("two","t2o"),
        ("three","t3e"),
        ("four","f4r"),
        ("five","f5e"),
        ("six","s6x"),
        ("seven","s7n"),
        ("eight","e8t"),
        ("nine","n9e"),
        //("zero","z0o"),
    ]);
    res.push_str(src);
    // min priority priority_queue
    let mut pmin_queue: DoublePriorityQueue<&str,usize>= DoublePriorityQueue::new();
    // find match and put words that matched with a lower index in a data structure to replace
    for word in screen_iter {
        if let Some(idx) = res.find(word) {
            // insert index into priority queue
            pmin_queue.push(word,idx);
        }
    }
    // go down data struct and replace from least to greatest
    while let Some((word,_)) = pmin_queue.pop_min() {
        res = res.replace(word,replace_ptrn[word]);
    }
    return res;
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
        let screened_str = screen_str(&buf); 
        let digits_str = match_finder(&screened_str);
        if digits_str == "" || digits_str == "00" {
            continue;
        } 
        let digits:i32 = digits_str.parse().expect("Error converting value: {digits_str}");
        println!("{}->{}",buf,digits);
        sum_vec.push(digits);
        buf.clear(); // prepare buffer for next line
        _count+=1;
    }
    // do rest of parsing
    sum_vec.into_iter().reduce(|sum,x| sum + x).expect("Err summing vec...")
}    
fn main() {
    let mut txt = String::new();
    std::fs::File::open("input2-1.txt").unwrap().read_to_string(&mut txt).expect("ERR:could not find input.txt file");
    let sum = parse_block(&mut txt.as_bytes());
    println!("Part 2 Sum: {}",sum)
}

#[cfg(test)]
mod test {
    use crate::*;
    #[test]
    fn input_1(){
        let mut txt = String::new();
        std::fs::File::open("input.txt").unwrap().read_to_string(&mut txt).expect("ERR:could not find input.txt file");
        assert_eq!(142,parse_block(&mut txt.as_bytes()));

    }
    #[test]
    fn input_2(){
        let mut txt = String::new();
        std::fs::File::open("input2.txt").unwrap().read_to_string(&mut txt).expect("ERR:could not find input.txt file");
        assert_eq!(281,parse_block(&mut txt.as_bytes()));
    }

    #[test]
    fn input_3(){
        let mut txt = String::from("eightwo\n");
        assert_eq!(78,parse_block(&mut txt.as_bytes()));
    }
}
