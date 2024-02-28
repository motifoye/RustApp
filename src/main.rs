use crate::group_m::*;
use crate::group_n::*;
use crate::io_worker::{clear, pause, read_line};

mod config;
mod io_worker;
mod group_m;
mod group_n;

static mut CURRENT_POSITON: Navigate = Navigate::Home;
fn main() {
    config::init();

    loop {
        clear();

        unsafe {
            match CURRENT_POSITON {
                Navigate::Home => { menu_home() }
                Navigate::GroupM => { group_m() }
                Navigate::GroupN => { group_n() }
                Navigate::Exiting => { break }
            }
        }

        pause();
    }
}

#[derive(Debug)]
enum Navigate {
    Home,
    GroupM,
    GroupN,
    Exiting,
}
fn menu_home() {
    println!("0: Выход");
    println!("1: Практическое занятие №3 Тема: «Работа с файлами и потоками ввода-вывода.»");
    println!("2: Практическое занятие №4 Тема: «Работа с динамической памятью и управление ею»");
    match read_line().trim() {
        "0" => unsafe {CURRENT_POSITON=Navigate::Exiting},
        "1" => unsafe { CURRENT_POSITON = Navigate::GroupM },
        "2" => unsafe { CURRENT_POSITON = Navigate::GroupN },
        _ => {println!("Недопустимый вариант")}
    }
}

fn group_m(){
    println!("0: Назад");
    println!("1: Запись строк в файл циклом");
    println!("2: Сумма чисел из файла");
    println!("3: Копировать содерзимое из одного файла в другой");
    println!("4: Поиск длинного слова в файле");
    println!("5: Считает в файле строки, слова, символы");
    match read_line().trim() {
        "0" => unsafe {CURRENT_POSITON=Navigate::Home},
        "1" => task_m1(),
        "2" => task_m2(),
        "3" => task_m3(),
        "4" => task_m4(),
        "5" => task_m5(),
        _ => {println!("Недопустимый вариант")}
    }
}

fn group_n(){
    println!("0: Назад");
    println!("1: Динамический массив");
    match read_line().trim() {
        "0" => unsafe {CURRENT_POSITON=Navigate::Home},
        "1" => task_n1(),
        _ => {println!("Недопустимый вариант")}
    }
}