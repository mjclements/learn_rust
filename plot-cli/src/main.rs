use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
fn main() {
    let f = File::open("input.txt").unwrap();
    let mut reader = BufReader::new(f);

    let mut a = [0];
    loop {
        let len = reader.read(&mut a).unwrap();
        for _ in 0..a[0] {
            print!(".");
        }
	    println!("");
        if len == 0 {
            break;
        }
    }
}
