use std::fs::File;
use std::io::{Read, stdin, stdout};
use std::io::Write;
use std::path::Path;
use std::process::Command;

pub fn read_file_all(path: &str) -> String {
    let mut file = File::options()
    .read(true)
    .open(path)
    .expect("[E] fn: read_file_all, can't open file");

    let mut buf = String::new();
    let _ = file.read_to_string(&mut buf);

    buf
}

pub fn write_to_file(path: &str, buf: String) {
    let mut file = File::options().append(true).open(path)
    .expect("[E] fn: write_to_file, can't open file");
    match write!(file, "{}", buf) {
        Ok(_) => println!("[I] write file "),
        Err(e) => println!("[E] write file | {:?}",e)
    }
}

pub fn read_line() -> String {
    let mut buf: String = String::new();
    stdin().read_line(&mut buf).unwrap();
    buf
}

pub fn print(string: &str) {
    stdout().write_all(string.as_bytes()).unwrap();
    stdout().flush().unwrap();
}

pub fn pause() {
    println!("\nEnter any key to continue...");
    read_line();
}

pub fn clear() {
    Command::new("cmd")
        .args(["/c", "cls"])
        .spawn()
        .expect("cls command failed to start")
        .wait()
        .expect("failed to wait");
}