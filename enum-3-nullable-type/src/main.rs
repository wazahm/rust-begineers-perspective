//  Nullable Types using Enum

#![allow(dead_code, unused_variables, unused_assignments)]

enum NullableInt {
    Int(i32),
    Undefined
}

fn main() {
    let mut a = NullableInt::Undefined;

    print_nullable_int(&a);

    a = NullableInt::Int(100);
    incr_int(&mut a);

    print_nullable_int(&a);
}

fn print_nullable_int(x: &NullableInt) {
    if let NullableInt::Int(value) = x {
        println!("\n Value is {}", value);
    } else {
        println!("\n Value is `Undefined`");
    }
}

fn incr_int(x: &mut NullableInt) {
    match x {
        NullableInt::Int(value) => { *value  = *value + 1; },
        NullableInt::Undefined => {}
    }
}
