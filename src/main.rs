use std::io::{BufRead, Read};
use regex::Regex;
// block can either be big string of text or a file handle
// (any place text can come from)
// remake to use recursion
fn parse_block(block:&mut dyn BufRead) {
    let re = match Regex::new(r"(\d).*?(\d)") {
        Ok(x) => x,
        Err(e) => panic!("Failed to build digit parsing regex.\n{e}")
    };
    let mut buf = String::with_capacity(64); // allocate one big size to minimize frequency of allocations
    //make a char buffer
    let mut count:usize = 0;  // # of reads
    while let Ok(n) = block.read_line(&mut buf) {
        if n == 0 {
            //print!("{}",count);
            break;
        }
        // handle line matching here
        let match_str = match re.captures(&buf) {
            Some(m) => {
                let (_,[x,y]) = m.extract();
                let mut x = String::from(x);
                x.push_str(y);
                x
            },
            None=>"".to_string()
        };
        println!("Digits:{match_str}");
        buf.clear(); // prepare buffer for next line
        count+=1;
    }
    // do rest of parsing
}    
fn main() {
    let mut txt = String::new();
    std::fs::File::open("input.txt").unwrap().read_to_string(&mut txt);
    parse_block(&mut txt.as_bytes());
}
