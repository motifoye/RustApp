use std::io;
use std::io::Write;
use std::fs;

// fn su(a:i32,b:i32) -> i32 {
//     a+b
// }

fn main() {
    
    io::stdout().write_all(b"::4514851481::\n").unwrap();
    // println!("{:?}",su(9,9));
    
    
    
    
}


fn create_vec() {
    let mut count = String::new();
    
    match io::stdin().read_line(&mut count) {
        Ok(n) => {
            println!("{n} bytes read");
            // println!("{count}");
        }
        Err(error) => println!("error: {error}"),
    }

    let count: usize = count.trim().parse().expect("err: parse input");
    
    let mut von = vec![0;count];
    for i in 0..count{
        von[i] = i;
    }
    
    println!("{:?}",von);
}