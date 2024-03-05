use regex::Regex;
// cube conudrum
struct RGB {
    r: u8,
    g: u8,
    b: u8,
}
const max_rgb: RGB = RGB { // max rgb value acceptable
   r:  12,
   g:  13,
   b:  14,
};
const dbg:bool = false; // set true to print additional logs
fn unnamed_func(input:&str) {
    let mut counter = 0;
    let game_pattern = Regex::new(r"^Game\s+\d+:(\s+\d+\s+(red|green|blue),?);?").unwrap();
    //let color_pattern = Regex::new(r"(\s+(\d+)\s+(red|green|blue),?);?").unwrap();
    let color_pattern = Regex::new(r"(\s+(\d+)\s+(red|green|blue),?;?)").unwrap(); // counts semi
                                                                                   // colon
                                                                                   // seperator
                                                                                   // in captures
    let validate_line = |line:&str| game_pattern.is_match(line);
    // iterate over valid lines
    input.split("\n").into_iter().filter( |line| validate_line(line)).filter(|&line| {
        counter += 1;
        if dbg {
            println!("Line {}: {:?}",counter,line);
        }
        //for x in color_pattern.find_iter(line).map(|m| m.as_str()) {
        for (cap, [_,num,color]) in color_pattern.captures_iter(line).map(|m| m.extract()) {
            let num_u8:u8 = num.parse().expect("num wasnt u8");
            if dbg {
                println!("Full Capture: {}\tNum:{}\tColor:{}",cap,num,color);
            }
            match color.to_ascii_lowercase().as_str() {
                "red" if num_u8 > max_rgb.r => return false,
                "blue" if num_u8 > max_rgb.b => return false,
                "green" if num_u8 > max_rgb.g => return false,
                "green" | "red" | "blue" => true,
                _ => return false,
            };
        }
        true
    }).for_each(|line| {
        println!("Valid: {}",line);
        // add up game ids
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
