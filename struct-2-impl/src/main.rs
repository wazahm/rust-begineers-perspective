#![allow(dead_code, unused_variables)]

// Struct Impl Example

enum Designation {
    CEO,
    VP,
    Manager,
    TeamLead,
    SDE2,
    SDE1,
    Intern
}

struct Employee {
    name: String,
    age: u8,
    designation: Designation 
}

impl Employee {
    fn designation_str(&self) -> String {
        let mut ret_val = String::new();

        match self.designation {
            Designation::CEO => { ret_val.push_str("Chief Executive Officer"); },
            Designation::VP => { ret_val.push_str("Vice President"); },
            Designation::Manager => { ret_val.push_str("Manager"); },
            Designation::TeamLead => { ret_val.push_str("Team Leader"); },
            Designation::SDE2 => { ret_val.push_str("Senior Software Developer"); },
            Designation::SDE1 => { ret_val.push_str("Sofware Developer"); },
            Designation::Intern => { ret_val.push_str("Intern"); },
        }
        return ret_val;
    }
}

fn main() {
    let emp1 = Employee {
        name: String::from("Sridhar"),
        age: 35,
        designation: Designation::Manager
    };

    println!("\n {} is a {}", emp1.name, emp1.designation_str());
}