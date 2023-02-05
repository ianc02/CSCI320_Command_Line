use std::ffi::CString;
use std::fs;
use std::env;
use std::io::Write;
use anyhow;
use std::path::Path;
use nix::{sys::wait::waitpid,unistd::{fork, ForkResult, execvp}};



fn main() {

    loop {
        // https://doc.rust-lang.org/std/env/fn.current_dir.html
        let path = env::current_dir().unwrap();
        println!("The current directory is: {}", path.display());
        match line_input(){
            Ok(keep_going) => {
                if !keep_going {
                    break;
                }
            }
            Err(e) => {
                println!("Error: {e}");
            }
        }
    
    
    }
    
}

//https://github.com/gjf2a/shell/blob/master/src/bin/typing_demo.rs
fn line_input() -> anyhow::Result<bool>{
    let mut user_input = String::new();
    print!("$ ");
    std::io::stdout().flush()?;
    std::io::stdin().read_line(&mut user_input)?;
    if user_input.trim() =="exit"{
        Ok(false)
    }
    else{
        // user input check here
        
        let words: Vec<&str> = user_input.split_whitespace().collect();
        if words.is_empty(){return Ok(true)}
        else if words[0] == "cd"{
            //https://doc.rust-lang.org/std/env/fn.set_current_dir.html
            let root = Path::new(words[1]);
            assert!(env::set_current_dir(&root).is_ok());
            println!("Changed directory to {}", root.display());
            return Ok(true);
        }
        else{
            //https://github.com/gjf2a/shell/blob/master/src/bin/fork_ls_demo.rs
            match unsafe{fork()}{
                Ok(ForkResult::Parent { child, .. })=> {
                    waitpid(child, None).unwrap();
                }
                Ok(ForkResult::Child)=> {
                    let cmd = externalize(&user_input);
                    match execvp::<CString>(cmd[0].as_c_str(),&cmd){
                        Ok(_) => {println!("Child finished");},
                        Err(e) => {println!("Could not execute: {e}");},
                    }
                }
                Err(_) => println!("Fork Failed"),
            }
            
        }
        
        Ok(true)
        // println!("You entered: {words:?}");
        // Ok(true)
    }
}

fn externalize(command: &str) -> Vec<CString> {
    command.split_whitespace()
        .map(|s| CString::new(s).unwrap())
        .collect()
}

