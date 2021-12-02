
#![feature(stdin_forwarders)]
use std::io;

fn main() {
    let lines = io::stdin().lines();
    let mut last = 0;
    let mut first = true;
    let mut num_increases = 0;
    for line in lines {
        let i = line.unwrap().parse::<i32>().unwrap();
        println!("Input: {}", i);

        if first {
            first = false;
        } else {
            if i > last {
                num_increases += 1;
            }
        }
        last = i
    }
    println!("Number of increases: {}", num_increases);
}
