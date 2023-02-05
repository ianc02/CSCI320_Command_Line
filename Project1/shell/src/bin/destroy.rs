use std::fs;

fn main() {
    
    
    for arg in std::env::args() {
        fs::remove_file(arg).unwrap();
    }
}
