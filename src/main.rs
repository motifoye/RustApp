use std::io::{stdin, Read};
use std::fs::File;
use std::io::Write;



const FILE_PATH_1: &str = "C:\\Users\\User\\Desktop\\doc_1.txt";
const FILE_PATH_2: &str = "C:\\Users\\User\\Desktop\\doc_2.txt";


fn main() {
    task_m2();
}

fn task_m2() {
    println!("\n\n::: TASK m2 :::\n\n");
    
    let buf = read_file_all(FILE_PATH_2);
    if buf.is_empty() {
        println!("[ERR] Empty file");
        return;
    }
    let buf = buf.trim().split("\n");

    let mut sum: i32 = 0;

    for i in buf {
        match i.trim().parse::<i32>() {
            Ok(n) => sum += n,
            Err(e) => println!("[ERR] fn: task_m2, {}, '{}'", e, i)
        }
    }

    println!("{}", sum);
    println!("\n\n::: FINISH TASK m2 :::\n\n")
}

fn task_m1() {
    println!("\n\n::: TASK m1 :::\n\n");
    loop {
        let buf = read_line();
        if buf.trim().eq("q") {
            println!("[dbg] input equal 'q'");
            break;
        }
        write_to_file(FILE_PATH_1, buf);
    }
    println!("-------------\n{}\n------------\n", 
    read_file_all(FILE_PATH_1));
    println!("\n\n::: FINISH TASK m1 :::\n\n");
}

fn sum(a:i32,b:i32) -> i32 {  
    a+b
}

fn read_file_all(path: &str) -> String {
    let mut file = File::options()
    .read(true)
    .open(path)
    .expect("[ERR] fn: read_file_all, can't open file");

    let mut buf = String::new();
    let _ = file.read_to_string(&mut buf);

    buf
}

fn write_to_file(path: &str, buf: String) {
    let mut file = File::options().append(true).open(path)
    .expect("[ERR] fn: write_to_file, can't open file");
    match write!(file, "{}", buf) {
        Ok(_) => println!("[OK] write file "),
        Err(e) => println!("[ERR] write file | {:?}",e)
    }
}

fn read_line() -> String {
    let mut buf: String = String::new();
    stdin().read_line(&mut buf).unwrap();
    buf
}

fn create_vec() {
    let count: usize = read_line().trim().parse()
    .expect("[ERR] fn: create_vec, can't parse input");
    
    let mut von = vec![0;count];
    for i in 0..count{
        von[i] = i;
    }
    
    println!("{:?}",von);
}