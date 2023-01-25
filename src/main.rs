#![allow(dead_code)]

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: f32,
    y: f32
}

#[derive(Debug, Clone, Copy)]
struct Line {
    a: f32,
    b: f32,
    c: f32
}

impl Point {
    fn new() -> Self {
        Point {
            x: 0.0,
            y: 0.0
        }
    }
}

impl Line {
    fn new() -> Self {
        Line {
            a: 0.0,
            b: 0.0,
            c: 0.0
        }
    }
}
//Функция для составление линейного уравнения Ax+By-C=0
fn find_line_equation(x1: f32, y1: f32, x2: f32, y2: f32) -> (f32, f32, f32){
    let mut equation_line = Line::new();
    equation_line.a = y2 - y1;
    equation_line.b = x1 - x2;
    equation_line.c = x2*y1 - x1*y2;
    return(equation_line.a, equation_line.b, equation_line.c);
}

fn find_point_intersect(first_line: Line, second_line: Line) -> (f32, f32, bool){
    let system_det = (first_line.a*second_line.b - second_line.a*first_line.b).abs();
    let have_point = system_det > 0.0001;
    if !have_point { return (0.0,0.0, false); }
    //После проверки на параллельность прямых ищем точку пересечения
    let mut point_result = Point::new();
    point_result.x = (first_line.c*second_line.b - second_line.c*first_line.b) / system_det;
    point_result.y = (first_line.a*second_line.c - second_line.a*first_line.c) / system_det;
    return(point_result.x, point_result.y, true);
}

// Открываем файл ,возвращаем итератор для построчного чтения файла.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
//Проверка, дежит ли точка пересечения на отрезке
fn is_between(x: f32, y:f32, x1: f32, y1: f32, x2: f32, y2: f32) -> bool{
    let dxc = x - x1;
    let dyc = y - y1;
    let dxl = x2 - x1;
    let dyl = y2 - y1;

    let cross = dxc * dyl - dyc * dxl;
    if cross.abs() > 0.01 {return false};

    if dxl.abs() >= dyl.abs() {
        if dxl > 0.0 {return x1 <= x && x <= x2} else {return x2 <= x && x <= x1}
    } else {
        if dyl > 0.0 {return y1 <= y && y <= y2} else {return y2 <= y && y <= y1}
    }
}

fn main() {
    let mut file_lines = Vec::new();

    //Открываем файл с заданной полупрямой и отрезками
    if let Ok(lines) = read_lines("./test1.txt") {
        // Построчно читаем файл
        for line in lines {
            if let Ok(ip) = line { file_lines.push(ip);}      
        }   
    }

    let mut points_array = Vec::new();

    //Разбираем строки из файла
    for i in 0..file_lines.len(){

        let mut file_string = file_lines[i].clone();
        //Готовим строку к разделению на числа
        file_string = file_string.replacen(" ", ",", 1).to_string();
        //делим строку на числа и переводим каждое число в f32 (флоат32)
        let string_value: Vec<&str> = file_string.split_terminator(',').collect();
        for j in 0..4{
            let number: f32 = string_value[j].parse().unwrap();
            points_array.push(number);
        }
    }

    let mut main_line = Line::new();
    //Находим линейное уравнение для полупрямой
    (main_line.a, main_line.b, main_line.c) = find_line_equation(points_array[0], points_array[1], points_array[2], points_array[3]);
    println!("Полупрямая, её точки ({},{}) и ({},{})",points_array[0], points_array[1], points_array[2], points_array[3]);
    //Для каждого отрезка находим пересечение
    for i in 1..file_lines.len() {
       
    let mut segment_line = Line::new();
    (segment_line.a, segment_line.b, segment_line.c) = find_line_equation(points_array[i * 4], points_array[i * 4 + 1], points_array[i * 4 + 2], points_array[i * 4 + 3]);
    //находим пересечение между прямой отрезка и полупрямой
    let intersect_answer = find_point_intersect(main_line, segment_line); 
    if intersect_answer.2 {println!("Отрезок ({},{}) и ({},{}) имеет точку пересечения в ({},{})!",points_array[i * 4], points_array[i * 4 + 1], points_array[i * 4 + 2], points_array[i * 4 + 3], intersect_answer.0, intersect_answer.1 );}
    else {println!("Отрезок ({},{}) и ({},{}) не имеет точку пересечения!",points_array[i * 4], points_array[i * 4 + 1], points_array[i * 4 + 2], points_array[i * 4 + 3]);}
    } 
}
