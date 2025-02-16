use std::{time::Instant, vec};

fn main() {
    let numbers = vec![1, 2, 4, 6, 10, 20];
    println!("Brute Force");
    let now = Instant::now();
    println!("Data -> {:?}", brute_force(&numbers));
    let elapsed = now.elapsed();

    println!("{:?} has elapsed...", elapsed);
    println!("-----");
    println!("Division Method");
    let now = Instant::now();
    println!("Data -> {:?}", division_method(&numbers));
    let elapsed = now.elapsed();

    println!("{:?} has elapsed...", elapsed);

    println!("-----");
    println!("Prefix & Suffix Method");
    let now = Instant::now();
    println!("Data -> {:?}", prefix_multiply(&numbers));
    let elapsed = now.elapsed();

    println!("{:?} has elapsed...", elapsed);

    println!("-----");
    println!("Prefix & Suffix Method Optimized");
    let now = Instant::now();
    println!("Data -> {:?}", prefix_multiply_optimized(&numbers));
    let elapsed = now.elapsed();

    println!("{:?} has elapsed...", elapsed)
}

fn brute_force(numbers: &Vec<i32>) -> Vec<i32> {
    let n = numbers.len();
    let mut products: Vec<i32> = vec![];
    for i in 0..n {
        let mut product = 1;
        for j in 0..n {
            if i == j {
                continue;
            }
            product *= numbers[j]
        }
        products.push(product);
    }

    return products;
}

fn division_method(numbers: &Vec<i32>) -> Vec<i32> {
    let n = numbers.len();
    let mut prod = 1;
    let mut zero_counter = 0;
    let mut products = vec![0; n];
    for i in 0..n {
        if numbers[i] == 0 {
            zero_counter += 1;
        } else {
            prod *= numbers[i]
        }
    }

    if zero_counter > 1 {
        return products;
    }

    for j in 0..n {
        if zero_counter > 0 {
            if numbers[j] == 0 {
                products[j] = prod;
            } else {
                products[j] = 0;
            }
        } else {
            products[j] = prod / numbers[j];
        }
    }

    return products;
}

fn prefix_multiply(numbers: &Vec<i32>) -> Vec<i32> {
    let n = numbers.len();
    let mut products = vec![0; n];
    let mut prefix = vec![1; n];
    let mut suffix = vec![1; n];

    for i in 1..n {
        prefix[i] = numbers[i - 1] * prefix[i - 1];
    }

    for j in (0..=n - 2).rev() {
        suffix[j] = numbers[j + 1] * suffix[j + 1];
    }
    for k in 0..n {
        products[k] = prefix[k] * suffix[k];
    }
    return products;
}

fn prefix_multiply_optimized(numbers: &Vec<i32>) -> Vec<i32> {
    let n = numbers.len();
    let mut products = vec![1; n];

    let mut prefix = 1;

    for i in 0..n {
        products[i] *= prefix;
        prefix *= numbers[i];
    }

    let mut postfix = 1;
    for j in (0..n).rev() {
        products[j] *= postfix;
        postfix *= numbers[j];
    }

    return products;
}
