extern crate rand;
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("guess the num!");
    println!("input your guess num~!");
    let mut guess=String::new();
    io::stdin().read_line(&mut guess)
    .expect("Failed to read line");

    println!("your num is:{}",guess);
    let guess : u32 =guess.trim().parse()
        .expect("please type an unsigned number!");

    let secret_no = rand::thread_rng().gen_range(1,101);
    println!("the secret_no is :{}",secret_no);
    match guess.cmp(&secret_no){
        Ordering::Less => println!("small!"),
        Ordering::Greater => println!("big!"),
        Ordering::Equal => println!("equal!"),
    }
}
