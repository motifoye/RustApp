use std::io::Write;


const FILE_PATH_1: &str = "C:\\Users\\User\\Desktop\\doc_1.txt";
const FILE_PATH_2: &str = "C:\\Users\\User\\Desktop\\doc_2.txt";


fn main() {
    std::io::stdout().write_all(b"::4514851481::\n").unwrap();
    println!("{}\n",sum(9,9));
    
    loop {
        let buf = read_line();
        
        // if buf.as_bytes().eq(&[113,13,10]) 
        // if buf.trim().as_bytes().eq(&[113]) 
        if buf.trim().eq("q") {
            println!("[dbg] input equal 'q'");
            break;
        }

        write_to_file(FILE_PATH_1, buf);
        
    }
}

fn sum(a:i32,b:i32) -> i32 {  
    a+b
}

fn write_to_file(path: &str, buf: String) {
    use std::fs::File;
    let mut file = File::options().append(true).open(path).expect("[ERR] open file");
    match write!(file, "{}", buf) {
        Ok(_) => println!("[OK] write file "),
        Err(e) => println!("[ERR] write file | {:?}",e)
    }
}

fn read_line() -> String {
    use std::io::stdin;
    let mut buf: String = String::new();
    stdin().read_line(&mut buf).unwrap();
    buf
}

fn create_vec() {
    let count: usize = read_line().trim().parse().expect("[ERR] fn: create_vec, can't parse input");
    
    let mut von = vec![0;count];
    for i in 0..count{
        von[i] = i;
    }
    
    println!("{:?}",von);
}