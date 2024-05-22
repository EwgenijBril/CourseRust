#![warn(clippy::all, clippy::pedantic)]
use std::io;

fn process(str: &str) -> u8 {
    str.trim().parse::<u8>().expect("Please type a number!")
}

fn main() {
    let mut user_choice: String = String::new();

    io::stdin().read_line(&mut user_choice).unwrap();

    let n_choice = process(&user_choice);
    process("test");

    println!("Number: {n_choice}");
    println!("String: {user_choice}")
    // scope
    //owership
    // borrowing
    // let a = 42;
    // let b = a;
    // demo(a); // copy
    // println!("{a}");

    // let mut s = String::from("hello");
    // s.push_str("world")

    // let mut a = 42;
    // let b = a;

    // a = a + 1;

    // let mut s1 = String::from("hello");
//  let (l, s1) = demo_str(s1); // move
    // let l = demo_str(& mut s1); // move

    // let s2 = s1;  //move
    // let s2 = s1.clone(); //clone 

    // let srt = "test"; //literal

    // let s1 = String::from("hello");

    // let s2 = &s1[0..3];

    // println!("{s2}")
}

// fn demo_str(s3: String) -> (usize, String) {
//     (s3.len(), s3)
// }

// fn demo_str(s3: & mut String) -> usize {
//     s3.push_str("world");
//     s3.len()
// }

// fn demo(a2: i32) {
//     // a2 + 2;
// }
