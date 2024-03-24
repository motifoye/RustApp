use rand::Rng;
use crate::config::PATH_FILE_4;
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
            print(format!("{:.4} ", matrix[i][j]).as_str());
        }
        println!();
    }
    println!("{matrix:?}")
}

/// запрашивает у пользователя количество студентов и их оценки.
/// Выделите динамическую память для массива оценок и
/// вычислите среднюю оценку.
/// Выведите среднюю оценку на экран.
pub fn task_n3() {
    print("количество студентов: ");
    let student_count = read_line().trim().parse::<usize>();
    if student_count.is_err() {
        return;
    }
    let student_count = student_count.unwrap();

    print("количество оценок: ");
    let rating_count = read_line().trim().parse::<usize>();
    if rating_count.is_err() {
        return;
    }
    let rating_count = rating_count.unwrap();

    let mut students: Vec<Student> = vec![];
    for _ in 0..student_count {
        let mut a = Student::new("Alex".to_string(), 23, vec![0; rating_count]);
        for j in 0..rating_count {
            a.rating[j] = rand::thread_rng().gen_range(2..=5);
        }
        students.push(a);
    }

    let mut average_rating_group: f32 = 0.0;
    for student in students {
        println!("{:?} average rating: {}", student.clone(), student.clone().get_average_rating());
        average_rating_group += student.get_average_rating();
    }
    println!("average group rating: {:.2}", average_rating_group / student_count as f32);
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct Student {
    name: String,
    age: u8,
    rating: Vec<i32>
}
impl Student {
    fn new(name: String, age: u8, rating: Vec<i32>) -> Self {
        Student {
            name,
            age,
            rating,
        }
    }
    fn get_average_rating(self) -> f32 {
        self.rating.clone().into_iter().sum::<i32>() as f32 / self.rating.len() as f32

    }
}

/// открывает файл и
/// считывает из него данные о студентах (имя, возраст, средние оценки и т.д.).
/// Динамически выделите память под массив студентов и
/// заполните его данными из файла.
/// Выведите информацию о студентах на экран.
pub fn task_n4() {
    let in_file_data = read_file_all(PATH_FILE_4);
    let in_file_data = in_file_data.split('\n');

    let mut students: Vec<Student> = vec![];
    for data in in_file_data {
        let t_student: Vec<String> = data.split(';').map(|e| e.trim().to_string()).collect();
        let student: Student = Student::new(
            t_student[0].clone(),
            t_student[1].clone().parse().unwrap(),
            t_student[2].clone().split(',').map(|e| e.trim().parse::<i32>().unwrap()).collect(),
        );
        println!("{:?}",student);
        students.push(student);
    }
}

/// считывает две строки с клавиатуры и
/// выделяет для них динамическую память.
/// Скопируйте первую строку во вторую строку с использованием указателей.
/// Выведите результат на экран.
pub fn task_n5() {
    print("input: ");
    let a = Box::new(read_line());
    print("input: ");
    let b = Box::new(read_line());
    let c = Box::new(format!("{}{}",*a.trim(),b.trim()));
    println!("{:?}",c);
}