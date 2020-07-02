// Enum variants can have type

#![allow(dead_code, unused_variables)]

enum NumStr {
    Num(i32),
    Str(String)
}

fn main() {
    let x = 5;
    let y = String::from("Hello");
    
    let mut z: NumStr = NumStr::Num(x);
    print_num_str(z);

    z = NumStr::Str(y);
    print_num_str(z);
}

fn print_num_str(x: NumStr) {
    match x {
        NumStr::Num(zz) => {
            println!("\n It is a Number - {}", zz);
        },
        NumStr::Str(value) => { println!("\n It is a String - {}", value); }
    }
}

//
// # Equivalent in C
// -----------------
//
//  void print_num_str(void* data, int type);
//
//