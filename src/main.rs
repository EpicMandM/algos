use rayon::prelude::*;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::sync::Mutex;

fn read_numbers_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<i64>> {
    let file = File::open(filename)?;
    let buf_reader = io::BufReader::new(file);
    buf_reader
        .lines()
        .map(|line| {
            line?.parse::<i64>()
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
        })
        .collect()
}

fn find_min(numbers: &[i64]) -> i64 {
    let min = Mutex::new(i64::MAX);
    numbers.par_iter().for_each(|&num| {
        let mut min_lock = min.lock().unwrap();
        if num < *min_lock {
            *min_lock = num;
        }
        drop(min_lock); // Explicitly drop the lock here
    });
    let min_value = *min.lock().unwrap(); // Get the value after the lock is reacquired
    min_value // Return the value, ensuring the lock is not in scope
}

fn find_max(numbers: &[i64]) -> i64 {
    let max = Mutex::new(i64::MIN);
    numbers.par_iter().for_each(|&num| {
        let mut max_lock = max.lock().unwrap();
        if num > *max_lock {
            *max_lock = num;
        }
        drop(max_lock); // Explicitly drop the lock here
    });
    let max_value = *max.lock().unwrap(); // Get the value after the lock is reacquired
    max_value // Return the value, ensuring the lock is not in scope
}

fn find_median(numbers: &mut [i64]) -> i64 {
    numbers.par_sort_unstable();
    let mid = numbers.len() / 2;
    if numbers.len() % 2 == 0 {
        (numbers[mid - 1] + numbers[mid]) / 2
    } else {
        numbers[mid]
    }
}

fn find_mean(numbers: &[i64]) -> f64 {
    let sum: i64 = numbers.par_iter().sum();
    sum as f64 / numbers.len() as f64
}

fn find_longest_sequences(numbers: &[i64]) -> (Vec<i64>, Vec<i64>) {
    let mut longest_increasing = vec![];
    let mut current_increasing = vec![];

    let mut longest_decreasing = vec![];
    let mut current_decreasing = vec![];

    if !numbers.is_empty() {
        current_increasing.push(numbers[0]);
        current_decreasing.push(numbers[0]);
    }

    for &num in numbers.iter().skip(1) {
        // Handle increasing sequence
        if num > *current_increasing.last().unwrap() {
            current_increasing.push(num);
        } else {
            if current_increasing.len() > longest_increasing.len() {
                longest_increasing = current_increasing.clone();
            }
            current_increasing.clear();
            current_increasing.push(num);
        }

        // Handle decreasing sequence
        if num < *current_decreasing.last().unwrap() {
            current_decreasing.push(num);
        } else {
            if current_decreasing.len() > longest_decreasing.len() {
                longest_decreasing = current_decreasing.clone();
            }
            current_decreasing.clear();
            current_decreasing.push(num);
        }
    }

    if current_increasing.len() > longest_increasing.len() {
        longest_increasing = current_increasing;
    }

    if current_decreasing.len() > longest_decreasing.len() {
        longest_decreasing = current_decreasing;
    }

    (longest_increasing, longest_decreasing)
}

fn main() {
    let filename = "10m.txt";
    let mut numbers = read_numbers_from_file(filename).expect("Could not read numbers from file");

    let min = find_min(&numbers);
    let max = find_max(&numbers);
    let median = find_median(&mut numbers);
    let mean = find_mean(&numbers);
    let (longest_increasing, longest_decreasing) = find_longest_sequences(&numbers);

    println!("Min: {}", min);
    println!("Max: {}", max);
    println!("Median: {}", median);
    println!("Mean: {}", mean);
    println!("Longest increasing sequence: {:?}", longest_increasing);
    println!("Longest decreasing sequence: {:?}", longest_decreasing);
}
