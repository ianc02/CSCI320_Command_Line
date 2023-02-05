use std::fs;


fn main(){
    for thing in fs::read_dir("./").unwrap() {
        let thing = thing.unwrap();
        let name = thing.path();
        println!("{}", name.display());
    }
}
