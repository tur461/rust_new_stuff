//use std::env;
use std::io;
use std::io::stdin;

fn main() {
    //let args: Vec<String> = env::args()::collect();

    println!("Enter numbers:");
    let mut buf = String::new();
    io::stdin()
        .read_line(&mut buf)
        .expect("expected a string of numbers");
    let num_arr: Vec<u64> = buf
        .trim()
        .split(" ")
        .map(|num_str| num_str.parse::<u64>().unwrap_or_default())
        .collect();

    println!("your input: {:?}", num_arr);
}
