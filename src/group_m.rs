use crate::config::{PATH_FILE_1, PATH_FILE_2};
use crate::io_worker::*;

pub fn task_m1() {
    println!("\n\n::: TASK m1 :::\n\n");
    loop {
        let buf = read_line();
        if buf.trim().eq("q") {
            println!("[I] m1: loop finish");
            break;
        }
        write_to_file(PATH_FILE_1, buf);
    }
    println!("-------------\n{}\n------------\n",
             read_file_all(PATH_FILE_1));
    println!("\n\n::: FINISH TASK m1 :::\n\n");
}

pub fn task_m2() {
    println!("\n\n::: TASK m2 :::\n\n");

    let buf = read_file_all(PATH_FILE_2);
    if buf.is_empty() {
        println!("[E] Empty file");
        return;
    }
    let buf = buf.trim().split("\n");

    let mut sum: i32 = 0;

    for i in buf {
        match i.trim().parse::<i32>() {
            Ok(n) => sum += n,
            Err(e) => println!("[E] fn: task_m2, {}: {}", e, i)
        }
    }

    println!("{}", sum);
    println!("\n\n::: FINISH TASK m2 :::\n\n")
}

pub fn task_m3() {
    println!("\n\n::: TASK m3 :::");
    println!("a: source, b: target\n\n");

    print("enter path a: ");
    let a = read_line();
    let a = a.trim();

    print("enter path b: ");
    let b = read_line();
    let b = b.trim();

    let mut buf = read_file_all(&a);
    write_to_file(&b, buf);

    println!("\n\n::: FINISH TASK m3 :::\n\n")
}
