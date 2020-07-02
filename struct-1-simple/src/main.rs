#![allow(dead_code, unused_variables)]

// Struct Simple Example

struct Employee {
    name: String,
    age: u8,
    designation: Designation 
}

enum Designation {
    CEO,
    VP,
    Manager,
    TeamLead,
    SDE2,
    SDE1,
    Intern
}

fn main() {
    let emp1: Employee = Employee {
        name: String::from("Sridhar"),
        age: 35,
        designation: Designation::Manager
    };

    let emp2: Employee = Employee {
        name: String::from("Vasanth"),
        age: 21,
        designation: Designation::Intern
    };

    println!("\n Name of emp1 - {}", emp1.name);
    println!("\n Age of emp1 - {}", emp1.age);
}