use std::{collections::HashSet, usize};

fn main() {
    let string = String::from("abbbcaddba");

    let value = SlidingWindows(string);
    println!("Output {}", value)
}

fn SlidingWindows(s: String) -> usize {
    let mut char_set = HashSet::<char>::new();
    let n = s.len();
    let mut left = 0;
    let mut longest = 0;
    for r in 0..n {
        let right_value = GetPointInString(&s, &r);
        loop {
            let left_value = GetPointInString(&s, &left);
            match char_set.get(&right_value) {
                None => break,
                Some(_) => {
                    char_set.remove(&left_value);
                    left += 1;
                }
            }
        }
        let w = (r - left) + 1;
        longest = max(&longest, &w);
        char_set.insert(right_value);
    }

    return longest;
}

fn max(a: &usize, b: &usize) -> usize {
    if a >= b {
        return *a;
    } else {
        return *b;
    }
}

fn GetPointInString(str: &String, index: &usize) -> char {
    let val = str.as_bytes()[*index];

    let c = val as char;

    return c;
}
