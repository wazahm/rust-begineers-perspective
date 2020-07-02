#![allow(dead_code, unused_variables, unused_assignments)]

//  Nullable Types using Enum

enum NullableInt {
    Int(i32),
    Undefined
}

fn main() {
    let mut a: NullableInt;
    
    a = NullableInt::Undefined;
    print_nullable_int(&a);

    a = NullableInt::Int(100);
    incr_int(&mut a);

    print_nullable_int(&a);
}

fn print_nullable_int(x: &NullableInt) {
    match x {
        NullableInt::Int(value) => { println!("\n Value is {}", value); },
        NullableInt::Undefined => { println!("\n Value is `Undefined`"); }
    }
}

fn incr_int(x: &mut NullableInt) {
    match x {
        NullableInt::Int(value) => { *value  = *value + 1; },
        NullableInt::Undefined => {}
    }
}

/*

struct Employee {
    name: String,
    age: u8
}

enum NullableEmployee {
    Valid(Employee),
    Null
}

*/