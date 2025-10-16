use prettytable::{Table, Row, Cell};

fn main() {
    let my_age = 33;
    let mut fav_num: u8= 8;
    fav_num = fav_num + 1;
    let str = "Ovo je string";
    let str = str.len();

    let broj8: u8 = 0xFF;
    let broj16: u16 = 0xFFFF;
    let broj32: u32 = 0xFFFFFF;
    let broj128: u128 = 0xFFFFFFFFFF;

    let float64 = 8.0;
    let float32: f32 = 8.0;

    let t = true;
    let f: bool = false;

    let slovo: char = 'L';
    let smile = '😄';

    let tup: (u16, u16, u16) = (8 , 7, 6);
    let (a, b, c) = tup;

    let student1 = ("Bob", 20, 8.5);
    let student2 = ("Boban", 21, 9.5);
    let student3 = ("Bobana", 22, 7.8);

    let students = vec![student1, student2, student3];

    let mut table = Table::new();

    table.add_row(Row::new(vec![
        Cell::new("Name"),
        Cell::new("Age"),
        Cell::new("Grade Average"),
    ]));

    for (name, age, grade) in &students {
        table.add_row(Row::new(vec![
            Cell::new(name),
            Cell::new(&age.to_string()),
            Cell::new(&format!("{:.2}", grade)),
        ]));
    }

    let total: f64 = students.iter().map(|(_, _, g)| g).sum();
    let avg_grade = total / students.len() as f64;

    println!("\nExercise 1");
    println!("My age is: {}", my_age);
    println!("Incremented fav number is: {}", fav_num);
    println!("Len of string: {}", str);

    println!("\nExercise 2");
    println!("Integer type 1: {}", broj8);
    println!("Integer type 2: {}", broj16);
    println!("Integer type 3: {}", broj32);
    println!("Integer type 4: {}", broj128);
    println!("Float ex 1: {}", float64);
    println!("Float ex 1: {}", float32);
    println!("Bool true: {}", t);
    println!("Bool false: {}", f);
    println!("Jedno slovo: {}", slovo);
    println!("Jedan smiley: {}", smile);

    println!("\nExercise 3");
    println!("Suma je : {}, prosecna vrednost: {}, a proizvod: {}", a+b+c, (a+b+c)/3, a*b*c);

    println!("\nExercise 4");
    table.printstd();
    println!("\nAverage grade of all students: {:.2}", avg_grade);
}
