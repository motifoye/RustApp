use rand::{random, Rng};
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

/// запрашивает у пользователя размерность матрицы и
/// выделяет динамическую память под эту матрицу.
/// Заполните матрицу случайными числами и
/// выведите ее на экран.
pub fn task_n2() {
    println!("::: TASK N2 :::");
    println!("размер матрицы");

    print("x: ");
    let x = read_line().trim().parse::<usize>();
    if x.is_err() {
        println!("[E] {:?}",x.unwrap_err());
        return;
    }
    let x = x.unwrap();

    print("y: ");
    let y = read_line().trim().parse::<usize>();
    if y.is_err() {
        println!("[E] {:?}",y.unwrap_err());
        return;
    }
    let y = y.unwrap();

    let mut matrix: Vec<Vec<i32>> = vec![vec![0;x];y];
    for i in 0..y {
        for j in 0..x {
            matrix[i][j] = rand::thread_rng().gen_range(1..=100);
            print(format!("{} ", matrix[i][j]).as_str());
        }
        println!();
    }
    println!("{matrix:?}")
}
