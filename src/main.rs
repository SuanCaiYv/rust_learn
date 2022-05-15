extern crate core;

mod base;

use crate::base::*;

fn main() {
    match error_result(false) {
        Ok(str) => {
            println!("{}", str)
        }
        Err(err) => {
            println!("err: {}", err)
        }
    }
}

pub fn error_result(flag: bool) -> Result<String, String> {
    if flag {
        Ok(String::from("success"))
    } else {
        Err(String::from("failed"))
    }
}
