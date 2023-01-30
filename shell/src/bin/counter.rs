use std::fs;
use std::env;


fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut dash = false;
    let mut w = true;
    let mut l = true;
    let mut c = true;
    let firstarg = &args[0];
    if firstarg.chars().next().unwrap() == '-'{
        dash = true;
        if !firstarg.contains('w'){w=false;}
        if !firstarg.contains('l'){l=false;}
        if !firstarg.contains('c'){c=false;}
    }
    for file in args{
        if dash{
            dash = false;
            continue;
        }
        let contents = fs::read_to_string(&file).expect("Usage: Something went wrong reading the file.");
        let iter_lines: Vec<&str> = contents.lines().collect();
        let mut wc = 0;
        let lc;
        let cc;
        let mut final_string = file.clone();
        final_string += ": ";
        if l{
            lc = iter_lines.len();
            final_string = final_string + "Length: {" + &lc.to_string() + "}. "
        }
        if w{
            for line in &iter_lines{
                for _word in line.split_whitespace(){
                    wc +=1;
                }
            }
            final_string = final_string + "Words: {" + &wc.to_string() + "}. "
        }
        if c{
            cc = contents.chars().count();
            final_string = final_string + "Characters: {" + &cc.to_string() + "}. "
        }
        println!("{final_string}");
        
        
        println!("")
    }
}
