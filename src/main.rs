use std::env;

fn main() {
    // get the arguments 
    let args : Vec<String> = env::args().collect();

    // iterate over the slice starting at the second argument
    // (This is at index 1, index 0 is the name of the executable)
    for a in &args[1..] {
        print!("{} ", a);
    }
    println!();
}
