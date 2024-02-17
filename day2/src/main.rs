use regex::Regex;

// cube conudrum
fn unnamed_func(input:&str) {
    let mut counter = 0;
    let game_pattern = Regex::new(r"^Game\s+\d+:(\s+\d+\s+(red|green|blue),?);?").unwrap();
    //let color_pattern = Regex::new(r"(\s+(\d+)\s+(red|green|blue),?);?").unwrap();
    let color_pattern = Regex::new(r"(\s+(\d+)\s+(red|green|blue),?;?)").unwrap(); // counts semi
                                                                                   // colon
                                                                                   // seperator
                                                                                   // in captures
    let validate_line = |line:&str| game_pattern.is_match(line);
    //let valid_lines: Vec<&str> = 
    input.split("\n").into_iter().filter( |line| validate_line(line)).for_each(|line| -> () {
        counter += 1;
        println!("Line {}: {:?}",counter,line);
        //for x in color_pattern.find_iter(line).map(|m| m.as_str()) {
        for (cap, [_,num,color]) in color_pattern.captures_iter(line).map(|m| m.extract()) {
            println!("Full Capture: {}\tNum:{}\tColor:{}",cap,num,color);
        }
    });
}
fn main() {
    unnamed_func(&
                 {
                     let mut x = String::from(std::fs::read_to_string("input.txt").unwrap());
                     x.push_str("Heya"); // this is a test string, it shouldn't be present in
                                         // parsing because filtering will get it out 
                    x
                 });
}
