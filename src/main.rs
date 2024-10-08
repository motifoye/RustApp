use crate::group_m::*;
use crate::group_n::*;
use crate::group_p::*;
use crate::io_worker::{clear, pause, read_line};

mod config;
mod io_worker;
mod group_m;
mod group_n;
mod group_p;

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
                Navigate::GroupP => { group_p() }
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
    GroupP,
    Exiting,
}
fn menu_home() {
    println!("0: Выход");
    println!("1: Практическое занятие №3 Тема: «Работа с файлами и потоками ввода-вывода.»");
    println!("2: Практическое занятие №4 Тема: «Работа с динамической памятью и управление ею»");
    println!("3: Анализ текста");
    match read_line().trim() {
        "0" => unsafe { CURRENT_POSITON=Navigate::Exiting },
        "1" => unsafe { CURRENT_POSITON = Navigate::GroupM },
        "2" => unsafe { CURRENT_POSITON = Navigate::GroupN },
        "3" => unsafe { CURRENT_POSITON = Navigate::GroupP },
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
    println!("2: Динамическая матрица");
    println!("3: Студенты, оценки в массиве");
    println!("4: Инфо о студентах в файле");
    match read_line().trim() {
        "0" => unsafe {CURRENT_POSITON=Navigate::Home},
        "1" => task_n1(),
        "2" => task_n2(),
        "3" => task_n3(),
        "4" => task_n4(),
        "5" => task_n5(),
        _ => {println!("Недопустимый вариант")}
    }
}

fn group_p(){
    println!("0: Назад");
    println!("1: ...");
    match read_line().trim() {
        "0" => unsafe {CURRENT_POSITON=Navigate::Home},
        "1" => task_p1(),
        _ => {println!("Недопустимый вариант")}
    }
}