use std::io;

fn main() {
    question_1();
    question_2a();
    question_2b();
    question_2c();
    question_3();
    question_4();
    question_5();
}

fn question_1() {
    let number: i32 = 11;
    let mut counter: i32 = 1;

    for i in 1..number {
        if number % i == 0 {
            counter = counter + 1;
        }
    }

    if counter == 2 {
        println!("Number is prime\n");
    }    else {
        println!("Not prime\n");
    }
}

fn question_2a() {
    let mut counter = 0;

    loop {
        counter += 1;
        
        if counter == 4 {
           break counter;
        }
        println!("I love my mother");
    };
    println!("I love my father\n");
}

fn question_2b() {
    for number in 0..3 {
        println!("I love my mother");
    }
    println!("I love my father\n");
}

fn question_2c() {
    let mut counter = 0;

    while counter != 3 {
        counter += 1;
        println!("I love my mother");
    }
    println!("I love my father\n");
}

fn question_3() {
    let number = 10;

    match number {
        10 => println!("Decade\n"),
        100 => println!("Century\n"),
        1000 => println!("Millennium\n"),
        _ => println!("Enter 10, 100 or 1000\n"),
    }

}

fn question_4() {
    let mut sum: i32 = 0;

    for number in 1..11 {
        sum = sum + number;
        }
    println!("Sum of first 10 natural number is : {}\n", sum);
}

fn question_5() {
    
    let marks_math = 65;
    let marks_physics = 55;
    let marks_chemistry = 50;
    let minimum_marks = 180;
    let minimum_subjects_maths = 140;
    
    let mut total_marks = 0;
    

    println!("Enter marks in maths");
    let mut maths = String::new();
    io::stdin().read_line(&mut maths).expect("Expect a number");

    let maths: u32 = maths.trim().parse().expect("Please type a number");
    
    println!("Enter marks in physics");
    let mut physics = String::new();
    io::stdin().read_line(&mut physics).expect("Expect a number");

    let physics: u32 = physics.trim().parse().expect("Please type a number");

    
    println!("Enter marks in chemistry");
    let mut chemistry = String::new();
    io::stdin().read_line(&mut chemistry).expect("Expect a number");

    let chemistry: u32 = chemistry.trim().parse().expect("Please type a number");

    let total_marks = maths + chemistry + physics;
    let subjects = chemistry + physics;
    let subject_maths = subjects + maths;

    if maths >= marks_math && physics >= marks_physics 
        && chemistry >= marks_chemistry && total_marks >= minimum_marks 
        || subject_maths >= minimum_subjects_maths{
        println!("Candidate is eligible for admission");
    } else {
        println!("Candidate is not eligible for admission");
    }

}