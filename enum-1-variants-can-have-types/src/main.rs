#![allow(dead_code, unused_variables)]

// Enum variants can have type

enum NumStr {
    Num(i32),
    Str(String)
}

fn main() {
    let x: i32 = 5;
    let y: String = String::from("Hello");
    
    let mut z: NumStr = NumStr::Num(x);
    print_num_str(z);

    z = NumStr::Str(y);
    print_num_str(z);
}

fn print_num_str(x: NumStr) {
    match x {
        NumStr::Num(int_val) => {
            println!("\n It is a Number - {}", int_val);
        },
        NumStr::Str(str_val) => {
            println!("\n It is a String - {}", str_val);
        }
    }
}