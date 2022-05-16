struct Solution {}

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.len() == 0 {
            return String::new();
        }
        let mut min_len = strs[0].len();
        for i in 1..strs.len() {
            min_len = min_len.min(strs[i].len());
        }
        if min_len == 0 {
            return String::new();
        }
        let mut idx: i32 = -1;
        for i in 0..min_len {
            let mut flag = true;
            for j in 1..strs.len() {
                if strs[0].bytes().nth(i).unwrap() != strs[j].bytes().nth(i).unwrap() {
                    flag = false;
                    break;
                }
            }
            if !flag {
                break;
            }
            idx = i as i32;
        }
        strs[0][0..(idx+1) as usize].to_string()
    }
}

trait Test {}

#[derive(Debug)]
struct Type {}

impl Test for Type {}

fn main() {
    let v = f();
    print_type_of(&v);
    println!("{}", Solution::longest_common_prefix(vec!["a".to_string(), "".to_string()]));
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn f() -> impl Test {
    Type {}
}
