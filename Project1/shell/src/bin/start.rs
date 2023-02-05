use std::fs;
use std::env;


fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let nums: i32;
    let firstarg = &args[0];
    let mut dash = false;
    if firstarg.chars().next().unwrap() == '-'{
        dash = true;
        let (_, n) = firstarg.split_at(1);
        nums = n.parse().unwrap();
    }
    else{nums = 10;}
    for file in args{
        if dash{
            dash = false;
            continue;
        }
        println!("{}", file);
        println!("");
        let contents = fs::read_to_string(file).expect("Usage: Something went wrong.");
        let mut i =0;
        for line in contents.split("\n"){
            println!("{line}");
            i +=1;
            if i == nums{
                break;
            }
        }
        println!("");
        println!("");
    }
    
}

