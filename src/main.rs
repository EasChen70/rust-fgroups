use std::io::{self, BufRead};
use std::collections::HashMap;
use std::fs::File;

fn main() {
    let mut fname = String::new();
    io::stdin().read_line(&mut fname).unwrap();
    // Remove the newline character from the filename
    let fname = fname.trim();
    //Open, and reads file, assigning to input
    let file = File::open(fname).expect("Failed to open input.txt");
    let input = io::BufReader::new(file);
    //Initialize hashmap
    let mut fingerprint_map: HashMap<String, Vec<String>> = HashMap::new();
    //loop over lines of input to process
    for line in input.lines(){
        if let Ok(line) = line{
        
            //Splits fingerprint and name
            let mut split_input = line.splitn(2, ' ');
            //Option, assigns fingerprint(key) and name(value), to substrings of split
            if let (Some(fingerprint), Some(name)) = (split_input.next(), split_input.next()){

                //insert key-value pairs into hashmap
                fingerprint_map.entry(fingerprint.to_string())
                .or_insert_with(Vec::new).push(name.to_string());
            }
        }
    }
    
    //printing function, enumerate allows us to track of index as well
    for (index, (_key, values)) in fingerprint_map.iter().enumerate(){
        if values.len() > 1{
            for(_index, value) in values.iter().enumerate(){
                println!("{}", value);            
            }
            //checks if any more groups left, if its not last one, print a blank space
            if index < fingerprint_map.len() - 1{
                println!();
            }    
        }
    }
}
