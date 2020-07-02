#![allow(dead_code, unused_variables)]

// Standart library Result<T, E> Enum

// # std::result::Result 
// ---------------------
// 
//  enum Result<X, Y> {
//      Ok(X),
//      Err(Y)
//  }

fn write_string_to_txt_file(file_name: String, data: String) -> Result<usize, String> {
    if file_name.ends_with(".txt") {
        /*
            1) open the file
            2) write the data to the file
            3) close the file
        */
        return Result::Ok(data.len());
    } else {
        let error_msg = String::from("Only .txt file extension is supported.");
        return Result::Err(error_msg);
    }
}

enum FileError {
    InvalidFile,
    PermissionDenied,
    UnexpectedEof
}

fn decrypt(file_name: String) -> Result<Vec<u8>, FileError> {
    let data: Vec<u8> = Vec::new();

    let file_not_exists = false;
    let permission_denied = false;
    let file_read = false;
 
    /*
        1) open the file
        2) read the data
        3) decrypt the content and store it in "data"
        4) close the file
    */

    if file_not_exists {
        return Result::Err(FileError::InvalidFile);
    } else if permission_denied {
        return Result::Err(FileError::PermissionDenied);
    } else if file_read {
        return Result::Err(FileError::UnexpectedEof);
    } else {
        return Result::Ok(data);
    }
}

fn main() {}