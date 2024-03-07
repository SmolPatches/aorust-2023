use regex::Regex;
// cube conudrum
struct RGB {
    r: usize,
    g: usize,
    b: usize,
}
const max_rgb: RGB = RGB { // max rgb value acceptable
   r:  12,
   g:  13,
   b:  14,
};
const dbg:bool = true; // set true to print additional logs
const verbose:bool = false; // set to true to print most logs

// given a str return the sum of the id's of valid game lines
pub fn unnamed_func(input:&str) -> usize {
    let mut counter = 0;
    let game_pattern = Regex::new(r"^Game\s+(?<id>\d)+:(\s+\d+\s+(red|green|blue),?);?").unwrap();
    //let color_pattern = Regex::new(r"(\s+(\d+)\s+(red|green|blue),?);?").unwrap();
    let color_pattern = Regex::new(r"(\s+(\d+)\s+(red|green|blue),?;?)").unwrap(); // counts semi
                                                                                   // colon
                                                                                   // seperator
                                                                                   // in captures
    let validate_line = |line:&str| game_pattern.is_match(line);
    let ans = input.split("\n").into_iter().filter( |line| validate_line(line)).filter(|&line| {
        counter += 1;
        if dbg && verbose{
            println!("Line {}: {:?}",counter,line);
        }
        //for x in color_pattern.find_iter(line).map(|m| m.as_str()) {
        for (cap, [_,num,color]) in color_pattern.captures_iter(line).map(|m| m.extract()) {
            let num_u8:usize = num.parse().expect("num wasnt usize");
            if dbg && verbose{
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
    // try reducing this to get a map
    }).fold(0,|id_sum,line| {
        // add up game id
        // get Some(Some(match)) to Some(match)
        // then map none to 0 and Some(match) to integer id value
        let id_int:usize = game_pattern.captures(line).and_then(|cap| {
            cap.name("id")
        }).map_or(0, |id| id.as_str().parse().expect("couldn't parse id"));
        if dbg {
            println!("Valid line: {} with id:{}",line,id_int);
            println!("{id_sum}+{id_int}:{}",id_sum+id_int);
        }
        id_sum + id_int
    });
    if dbg {
        println!("Result is: {}",ans);
    }
    ans
}
pub fn main() {
    println!("id sum:{}",unnamed_func(&
                 {
                     let mut x = String::from(std::fs::read_to_string("inputs/d2-input.txt").unwrap());
                     x.push_str("Heya"); // this is a test string, it shouldn't be present in
                                         // parsing because filtering will get it out 
                    x
                 }))
}
