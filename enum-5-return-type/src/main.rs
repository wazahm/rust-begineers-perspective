// Return types using Enum

#![allow(dead_code, unused_variables)]

enum FileOpResult {
    Success,
    Failure(String)
}

fn write_string_to_txt_file(file_name: String, data: String) -> FileOpResult {
    if file_name.ends_with(".txt") {
        /*
            1) open the file
            2) write the data to the file
            3) close the file
        */
        return FileOpResult::Success;
    } else {
        let error_msg = String::from("Only .txt file extension is supported.");
        return FileOpResult::Failure(error_msg);
    }
}

fn main() {
    let file1 = String::from("/home/debian/workspace/test-1.txt");
    let data1 = String::from("This is a test!!");

    let res1 = write_string_to_txt_file(file1, data1);
    match res1 {
        FileOpResult::Success => { println!("\n Operation 1 is a success"); },
        FileOpResult::Failure(x) => { println!("\n Operation 1 failed. Error - {}", x); }
    }

    let file2 = String::from("/home/debian/workspace/test-2");
    let data2 = String::from("This test should not pass");

    let res2 = write_string_to_txt_file(file2, data2);
    match res2 {
        FileOpResult::Success => { println!("\n Operation 2 is a success"); },
        FileOpResult::Failure(x) => { println!("\n Operation 2 failed. Error - {}", x); }
    }
}