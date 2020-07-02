//  Example of standard library Option<T> Enum

#![allow(dead_code, unused_variables, unused_assignments)]


// # std::option::Option 
// ----------------------
// 
//  enum Option<T> {
//      Some(T),
//      None
//  }

enum Option_i32 {
    Some(i32),
    None
}

enum Option_f32 {
    Some(String),
    None
}

fn main() {
    let nullable_int = Option_i32::None;
    /* After some lines of code.. */

    nullable_int = Option_i32::Some(100);

    
    let nullable_float = Option_f32::None;
    /* After some lines of code.. */

    nullable_float = Option_f32::Some(0.001);
}

fn function_recv_optional_values(x: i32, y: Option_i32) {
    /* code here */
}