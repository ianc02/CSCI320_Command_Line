use std::fs;
use std::env;


fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let mut allLines: Vec<String> = vec![];
    let mut firstarg = &args[0];
    let mut firstPass = false;
    let mut reverse = false;
    for file in &args{
        println!("{}",file);
        if !firstPass && firstarg.to_string() == "-r".to_string(){
            reverse=true;
            firstPass=true;
            continue;
        }
        if firstPass{
            let contents = fs::read_to_string(file).expect("Usage: Something went wrong.");
            for line in contents.split("\n"){
                allLines.push(line.to_owned());
            }
        }
        else{firstPass=true;}
    
    }
    allLines.sort();
    if reverse{
        
        for line in allLines.iter().rev(){
            println!("{}", line);
        }
    }
    else{
        for line in allLines.iter(){
            println!("{}", line);
        }
    }   
    
}

