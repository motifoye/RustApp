use std::arch::x86_64::_mm_pause;
use crate::group_m::*;
use crate::io_worker::{clear, pause, read_line};

mod group_m;
mod config;
mod io_worker;

fn main() {
    config::init();

    loop {
        clear();
        menu();
        match read_line().trim() {
            "0" => {break},
            "1" => task_m1(),
            "2" => task_m2(),
            "3" => task_m3(),
            "4" => task_m4(),
            "5" => task_m5(),
            _ => {println!("Недопустимый вариант")}
        }
        pause();
    }
}

fn menu() {
    println!("0: Выход");
    println!("1: Запись строк в файл циклом");
    println!("2: Сумма чисел из файла");
    println!("3: Копировать содерзимое из одного файла в другой");
    println!("4: Поиск длинного слова в файле");
    println!("5: Считает в файле строки, слова, символы");
}