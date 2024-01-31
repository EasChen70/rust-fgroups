use std::io::{self, BufRead};
use std::collections::HashMap;

fn main() {
    //Takes multilined input, and loops lines
    let input = io::stdin();
    for line in input.lock().lines(){
        if let Ok(line) = line{
            println!("{}", line);

            //Splits fingerprint and name
            let mut split_input = line.splitn(2, ' ');
            if let (Some(fingerprint), Some(name)) = (split_input.next(), split_input.next()){
                println!("Fingerprint: {}", fingerprint);
                println!("Name: {}", name);
            }
        }
    }


    
    let mut fingerprint_map: HashMap<String, Vec<String>> = HashMap::new();
}
