use std::io;
use std::vec::Vec;

#[derive(PartialOrd)]
#[derive(PartialEq)]
#[derive(Ord)]
#[derive(Eq)]
#[derive(Debug)]
struct Person {
    name: String,
    age: u32
}

fn main() {
    let mut vec: Vec<Person> = Vec::new();
    let mut run = true;
    while run {

        /* Name for the person */
        println!("Enter a name for the person");
        let mut n = String::new();
        io::stdin()
            .read_line(&mut n)
            .expect("Failed to read line error");
        let n = n.trim();

        /* Age for the person */
        println!("Enter an age for the person");
        let mut a = String::new();
        io::stdin()
            .read_line(&mut a)
            .expect("Failed to read line error");
        let a = a.trim();
        let a: u32 = a.parse().expect("Invalid Input");

        let person = Person {name: n.to_string(), age: a};
        vec.push(person);

        println!("Do you want to enter another person (y/n)");
        let mut yn = String::new();
        io::stdin()
            .read_line(&mut yn)
            .expect("Failed to read line error");
        let yn = yn.trim();
        if yn == "n" {
            run = false;
        }
    }

    println!("Sorted By Name");
    vec.sort();
    for i in &vec {
        println!("{:?}", i);
    }

    println!("\nSorted By Age:");
    vec.sort_by(|a, b| b.age.cmp(&a.age));
    for i in &vec {
        println!("{:?}", i);
    }
}