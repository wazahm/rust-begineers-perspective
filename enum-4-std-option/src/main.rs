// Standard library Option<T> Enum

#![allow(dead_code, unused_variables, unused_assignments)]


// # std::option::Option 
// ----------------------
// 
//  enum Option<T> {
//      Some(T),
//      None
//  }

fn main() {
    let nullable_int = Option::<i32>::None;
    /* After some lines of code.. */

    nullable_int = Option::Some(100);

    
    
    let nullable_float = Option::<f32>::None;
    /* After some lines of code.. */

    nullable_float = Option::Some(0.001);
}

fn function_recv_optional_values(x: i32, y: Option<i32>) {
    /* code here */
}

