use std::io::BufRead;
use crate::config::{PATH_FILE_1, PATH_FILE_2, PATH_FILE_3};
use crate::io_worker::*;

pub fn task_m1() {
    /// записывает введенные пользователем строки в файл.
    /// до тех пор, пока он не введет строку "конец".
    /// выведите содержимое файла на экран.

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
    /// открывает файл с числами, каждое число на отдельной строке.
    /// найдите их сумму.
    /// выведите на экран.

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
    /// копирует текст из одного файла в другой.
    /// Запрашивайте название исходного файла и файла назначения.
    /// скопируйте содержимое исходного файла в файл назначения.

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

pub fn task_m4() {
    /// открывает файл с набором слов.
    /// найдите самое длинное слово.
    /// Выведите это слово на экран.

    let buf = read_file_all(PATH_FILE_3);
    let buf = buf.trim();
    if buf.is_empty() {
        println!("[E] Empty file");
        return;
    }

    let words: Vec<_> = buf.trim().split(&[' ', '\n']).collect();
    let mut longest_word = "";
    for word in words {
        if word.len() > longest_word.len() {
            longest_word = word;
        }
    }
    print!("{}", longest_word);
}

pub fn task_m5() {
    /// открывает текстовый файл
    /// подсчитывает количество строк, слов и символов.
    /// Выведите результаты подсчета на экран.

    let buf = read_file_all(PATH_FILE_3);
    if buf.is_empty() {
        println!("[E] Empty file");
        return;
    }

    let lens = buf.split('\n').collect::<Vec<_>>().len();

    let words = buf.split(&['\n','\r',' ',',','.','-','—','–','«','»'])
        .filter(|i| !i.is_empty())
        .collect::<Vec<_>>()
        .len();

    let chars = buf.chars().count();

    print!("{:?} lines <> {:?} words <> {:?} chars",lens, words, chars);

}