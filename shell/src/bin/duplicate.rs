use std::fs;
use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3{
        println!("Usage: Need two arguments, the file name and the name you would like to replace it with. {}", &args[0])
    }
    else{
        let one = &args[1];
        let two = &args[2];
        match fs::copy(one, two){
            Ok(_)=>{}
            Err(e) => {println!("Error: {e}");}
        
        }
    }
}
