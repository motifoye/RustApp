use crate::io_worker::*;

/// запрашивает у пользователя количество элементов в массиве,
/// выделяет динамическую память для этого массива и
/// заполняет его значениями, введенными пользователем.
/// Затем выведите этот массив на экран.
pub fn task_n1() {
    println!("::: TASK N1 :::");
    println!("Ввод количество элементов");

    let count = read_line().trim().parse::<usize>();
    if count.is_err() {
        println!("[E] {:?}",count.unwrap_err());
        return;
    }
    let count = count.unwrap();

    let mut von = vec![String::new(); count];

    for i in 0..count {
        von[i] = read_line().trim().to_string();
    }

    println!("{:?}",von);
}