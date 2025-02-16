use std::{
    collections::HashMap,
    time::{self, Instant},
};

fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let target = 10;

    println!("HashMap----");
    let mut now = Instant::now();
    match two_sum(&numbers, target) {
        None => println!("Target Not Found"),
        Some([val, subs]) => println!("[{},{}] is the answer", val, subs),
    }
    let mut elapsed = now.elapsed();

    println!("Time Elapsed: {:?}", elapsed);

    println!("-----\n");
    println!("Two Pointers----");
    now = Instant::now();
    match two_sum_pointers(&numbers, target) {
        None => println!("Target Not Found"),
        Some([val1, val2]) => println!("[{},{}] is the answer", val1, val2),
    }
    elapsed = now.elapsed();
    println!("Time Elapsed: {:?}", elapsed);
}

fn two_sum(numbers: &Vec<i32>, target: i32) -> Option<[i32; 2]> {
    // if numbers len < 1 return None
    if numbers.len() <= 1 {
        return None;
    }

    // create a cache
    let mut cache = HashMap::<i32, i32>::new();

    for i in 0..numbers.len() {
        let subs = target - numbers[i];

        if let Some(&val) = cache.get(&numbers[i]) {
            return Some([val, numbers[i]]);
        }

        cache.insert(subs, numbers[i]);
    }

    return None;
}

fn two_sum_pointers(numbers: &Vec<i32>, target: i32) -> Option<[usize; 2]> {
    let mut l = 0;
    let mut r = numbers.len() - 1;
    while l < r {
        let cur_sum = numbers[l] + numbers[r];

        if cur_sum > target {
            r -= 1;
        } else if cur_sum < target {
            l += 1;
        } else {
            return Some([l + 1, r + 1]);
        }
    }

    return None;
}
