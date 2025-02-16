use std::time::Instant;

fn main() {
    let arr = vec![1, 2, -1, 2, 3];

    let mut start = Instant::now();
    let data = brute_force(&arr);
    println!("Elapsed time.. {:?}", start.elapsed());
    println!("{}", data);

    start = Instant::now();
    let data2 = kadane_method(arr);
    println!("Elapsed time.. {:?}", start.elapsed());
    println!("{}", data2)
}

// Maximum Sub Array Problem
// Suppose We have an array [1,-3,2,1,-1]
// We have to find the sub array that has the largest sum of numbers
// Sub array are continuos sequence of numbers in the array; i.e. It can be the whole array or for
// e.g. [1,-3,2] can be a sub array but [-3,-1] cannot be since it is not continuos....

fn brute_force(arr: &Vec<i32>) -> i32 {
    // Check each number in the array
    // Create all possible sub arrays
    // compare the sum of sub arrays

    // We can create a local maximum and then compare it with the global maximum
    let n = arr.len();

    let mut local_max: i32;
    let mut global_max: i32 = arr[0];

    for i in 0..n {
        local_max = arr[i];
        for j in 0..i {
            let local_arr = &arr[j..=i];
            let total = local_arr.iter().sum();

            if total > local_max {
                local_max = total;
            }
        }

        if local_max > global_max {
            global_max = local_max;
        }
    }

    return global_max;
}

fn kadane_method(arr: Vec<i32>) -> i32 {
    let n = arr.len();

    let mut local_max: i32;

    let mut global_max: i32;

    global_max = arr[0];
    local_max = arr[0];
    for i in 1..n {
        local_max = if arr[i] > arr[i] + local_max {
            arr[i]
        } else {
            arr[i] + local_max
        };

        if local_max > global_max {
            global_max = local_max;
        }
    }

    return global_max;
}
