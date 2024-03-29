use std::io::{self, BufRead};
use std::collections::HashMap;

fn main() {
    //Takes user input
    let input = io::stdin();
    //Initialize hashmap
    let mut fingerprint_map: HashMap<String, Vec<String>> = HashMap::new();
    //loop over lines of input to process
    for line in input.lock().lines(){
        if let Ok(line) = line{
        
            //Splits fingerprint and name
            let mut split_input = line.splitn(2, ' ');
            //Option, assigns fingerprint(key) and name(value), to substrings of split
            if let (Some(fingerprint), Some(name)) = (split_input.next(), split_input.next()){

                //insert key-value pairs into hashmap
                fingerprint_map.entry(String::from(fingerprint))
                .or_insert_with(Vec::new).push(String::from(name.trim()));
            }
        }
    }
    
    // Printing function to print only values with more than one occurrence
    for (index, values) in fingerprint_map.values().filter(|values| values.len() > 1).enumerate() {
        for value in values {
            println!("{}", value);
        }
        // Check if it's not the last group to print a blank line
        if index < fingerprint_map.values().filter(|values| values.len() > 1).count() - 1 {
            println!();
        }
    }

}