#![allow(dead_code)]

enum Stage {
    Beginner,
    Advanced,
}

enum Role {
    Student,
    Teacher,
}

fn main() {
    use crate::Stage::{Beginner, Advanced};
    use crate::Role::*;

    let stage = Beginner;
    let role = Student;

    match stage {
        Beginner => println!("Beginners are starting their learning journey!"),
        Advanced => println!("Advanced learners are mastering their subjects..."),
    }

    match role {
        Student => println!("Student are acquiring knowledge!"),
        Teacher => println!("Teachers are spreading knowledge!"),
    }
}
