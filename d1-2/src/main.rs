
#![feature(stdin_forwarders)]
use std::io;

fn main() {
    let lines = io::stdin().lines();
    let mut a = 0;
    let mut b = 0;
    let mut i = 0;
    let mut last = 0;
    let mut num_increases = 0;
    for line in lines {
        let x = line.unwrap().parse::<i32>().unwrap();
        println!("Input: {}", x);

        let sum = a + b + x;
        if i > 1 {
            println!("Window: {}", sum);
        }
        if i > 2 {
            if sum > last {
                num_increases += 1;
            }
        }
        a = b;
        b = x;

        last = sum;
        i += 1;
    }
    println!("Number of increases: {}", num_increases);
}
