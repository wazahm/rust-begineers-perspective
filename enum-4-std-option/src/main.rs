#![allow(dead_code, unused_variables, unused_assignments)]

// Standard library Option<T> Enum

struct Employee {
    name: String,
    age: u8
}

// # std::option::Option 
// ----------------------
// 
//  enum Option<T> {
//      Some(T),
//      None
//  }

fn main() {
    let mut nullable_int: Option<i32> = Option::None;

    /* After some lines of code.. */

    nullable_int = Option::Some(100);

    
    
    let mut nullable_float: Option<f32> = Option::None;
    /* After some lines of code.. */

    nullable_float = Option::Some(0.001);



    let mut nullable_emp: Option<Employee> = Option::None;
    /* After some lines of code.. */

    nullable_emp = Option::Some(Employee{
        name: String::from("Vasanth"),
        age: 21
    });
}

fn function_recv_optional_values(x: i32, y: Option<i32>) {
    /* code here */
}

