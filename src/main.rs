mod binary_tree;

use binary_tree::BinaryTree;
use std::io::{stdin};

#[derive(PartialOrd, PartialEq, Debug, Clone)]
struct User {
    year: u16,
    month: u8,
    day: u8,
    name: String
}

fn main() {
    let mut bt = BinaryTree::<User>::default();

    loop {
        let mut buffer = String::new();
        match stdin().read_line(&mut buffer) {
            Ok(status) => {
                if status == 0 {
                    break;
                }
                buffer = buffer.trim_end().to_owned();
                let mut input = buffer.split(" ");
                let operation = input.next().unwrap().trim();
                let year = input.next().unwrap().trim().parse().unwrap();
                let month = input.next().unwrap().trim().parse().unwrap();
                let day = input.next().unwrap().trim().parse().unwrap();
                let fname = input.next().unwrap().trim().to_owned();
                let lname = input.next().unwrap().trim();
                let user = User { year, month, day, name: fname + " " + lname };
                match operation {
                    "S:" => {
                        if bt.insert(user) {
                            println!("Stored");
                        }
                        else {
                            println!("Already stored");
                        }
                    },
                    "F:" => {
                        if bt.find(user) {
                            println!("Found");
                        }else {
                            println!("Not found");
                        }
                    },
                    "D:" => {
                        if bt.delete(user) {
                            println!("Deleted");
                        }
                        else {
                            println!("Not found");
                        }
                    },
                    _ => {
                        panic!("unknown operation");
                    }
                }
            },
            Err(_) => {
                panic!("error reading stdin");
            }
        };
    } 
}

