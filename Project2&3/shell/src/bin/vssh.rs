use std::{error::Error, process::id};
use std::ffi::CString;
use std::env;
use std::io::Write;
use anyhow;
use std::path::Path;

use nix::{unistd::{fork, pipe, close, dup2, execvp}, sys::{wait::waitpid, stat::Mode}, fcntl::{OFlag, open}};



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
    let mut new_user_input = String::new();
    let mut background = false;
    let mut pipe = false;
    let mut words: Vec<&str>;
    let mut out_filename: String= String::new();
    let mut in_filename: String= String::new();
    
    print!("$ ");
    std::io::stdout().flush()?;
    std::io::stdin().read_line(&mut user_input)?;
    if user_input.trim() =="exit"{
        Ok(false)
    }
    else{
        // user input check here
        user_input = user_input.trim().to_string();
        if user_input.is_empty(){return Ok(true)}
        else{
            background = false;
            pipe = false;
            if user_input.ends_with("&"){
                background = true;
                user_input.pop();
                
            }
            words = user_input.split_whitespace().collect();
            
            if words.len() > 2{
                let second = words[1].clone();
                let third = words[2].clone();
                let last =&words[words.len() - 1].clone();
                let second_to_last = words[words.len() - 2].clone();
                if second_to_last.contains(">") {
                    
                    out_filename = last.to_string();
                    words.pop();
                    words.pop();
                }
                if second.contains("<"){
                    in_filename = third.to_string();
                    words.remove(1);
                    words.remove(1);
                    let file_path = Path::new(in_filename.trim());
                    if !file_path.exists() {
                        println!("File {in_filename} does not exist");
                        return Ok(true)
                    }
                }

            }
            new_user_input = words.join(" ");
            if words[0] == "cd"{
                //https://doc.rust-lang.org/std/env/fn.set_current_dir.html
                let root = Path::new(words[1]);
                match env::set_current_dir(&root){
                    Ok(_) => {
                        println!("Changed directory to {}", root.display());
                    }
                    Err(e) => {println!("Error: Not a valid directory: {}, {}",e, root.display())}
                }
                
                
                
            }

            else{
                //https://github.com/gjf2a/shell/blob/master/src/bin/fork_ls_demo.rs
                match unsafe{fork()}{
                    Ok(nix::unistd::ForkResult::Parent { child, .. })=> {
                        
                        waitpid(child, None).unwrap();
                    }
                    Ok(nix::unistd::ForkResult::Child)=> {
                        if new_user_input.contains("|"){
                            pipeline(new_user_input.clone(),out_filename.clone(), in_filename.clone())
                        }
                        else{
                            if (background){
                                match nix::unistd::daemon(true, true){
                                    Ok(())=>{println!("PID is: {}",id())}
                                    Err(e)=>{println!("Daemon no work: {e}")}
                                }
                            }
                            let cmd = externalize(&new_user_input);
                            if !out_filename.is_empty(){
                                let flags: OFlag = [OFlag::O_CREAT, OFlag::O_WRONLY, OFlag::O_TRUNC].iter().copied().collect();
                                let mode: Mode = [Mode::S_IRUSR, Mode::S_IWUSR].iter().copied().collect();
                                let file_out = open(out_filename.trim(), flags, mode)?;
                                dup2(file_out, 1)?;
                            }
                            if !in_filename.is_empty(){
                                let flags = OFlag::O_RDONLY | OFlag::O_EXCL;
                                let mode = Mode::S_IRUSR;
                                let file_in = open(in_filename.trim(), flags, mode)?;
                                dup2(file_in, 0)?;
                            }
                            match execvp::<CString>(cmd[0].as_c_str(),&cmd){
                                Ok(_) => {println!("Child finished");},
                                Err(e) => {println!("Could not execute: {e}");},
                            }
                        }
                    }
                    Err(_) => println!("Fork Failed"),
                }
                
            }
        }
        Ok(true)
    }
}

fn pipeline(user_input: String, out_filename: String, in_filename: String){
    let mut cur_out = 1;
    if !out_filename.is_empty(){
        let flags: OFlag = [OFlag::O_CREAT, OFlag::O_WRONLY, OFlag::O_TRUNC].iter().copied().collect();
        let mode: Mode = [Mode::S_IRUSR, Mode::S_IWUSR].iter().copied().collect();
        let file_out = open(out_filename.trim(), flags, mode).unwrap();
        cur_out = file_out;
    }
    
    let mut commands: Vec<String> = Vec::new();
    for com in user_input.split("|"){
        commands.push(com.to_string());
    }
    commands.reverse();
    for (i,com) in commands.iter().enumerate(){
        if (i==commands.len()-1){continue;}
        let res = pipelineStageFunction(com.to_string(), cur_out);
        match res{
            Ok(value)=>{cur_out=value;}
            Err(e)=>{println!("An Error occured during piping: {}", e);}
        }
        
    }
    if !in_filename.is_empty(){
        let flags = OFlag::O_RDONLY | OFlag::O_EXCL;
        let mode = Mode::S_IRUSR;
        let file_in = open(in_filename.trim(), flags, mode).unwrap();
        dup2(file_in, 0);
    }
    dup2(cur_out, 1);
    let cmd = externalize(&commands[commands.len()-1]);
    
    match execvp::<CString>(cmd[0].as_c_str(),&cmd){
        Ok(_) => {println!("Child finished");},
        Err(e) => {println!("Could not execute: {e}");},
    }
    println!("is this reached");

}

fn pipelineStageFunction(command: String, cur_out: i32) -> Result<i32, Box<dyn Error>>{
    let cmd = externalize(&command);
    let (pipe_output, pipe_input) = pipe()?;
    match unsafe{fork()}{
        Ok(nix::unistd::ForkResult::Parent { child, .. })=> {
            
            close(pipe_input)?;
            dup2(pipe_output, 0)?;
            dup2(cur_out, 1)?;
            execvp(&cmd[0].as_c_str(), &cmd)?;

            //???
        }
        Ok(nix::unistd::ForkResult::Child)=>{
            close(pipe_output)?;
            return Ok(pipe_input);
        }
        Err(e) => return Err(Box::new(e)),

        
    }
    return Ok(1);
}

fn externalize(command: &str) -> Vec<CString> {
    command.split_whitespace()
        .map(|s| CString::new(s).unwrap())
        .collect()
}

