// Import necessary modules and types
use rayon::prelude::*;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::sync::Mutex;

// Function to read numbers from a file and return them as a vector of i64
fn read_numbers_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<i64>> {
    let file = File::open(filename)?;
    let buf_reader = io::BufReader::new(file);
    buf_reader
        .lines()
        // Parse each line as an i64, handling potential errors
        .map(|line| {
            line?.parse::<i64>()
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
        })
        .collect()
}

// Find the minimum value in a slice of i64 numbers
fn find_min(numbers: &[i64]) -> i64 {
    let min = Mutex::new(i64::MAX);
    numbers.par_iter().for_each(|&num| {
        let mut min_lock = min.lock().unwrap();
        if num < *min_lock {
            *min_lock = num;
        }
        // The lock is automatically dropped here as min_lock goes out of scope
    });
    // Explicitly create a new scope to ensure the lock is dropped before returning
    let min_value = *min.lock().unwrap();
    min_value // Now we are returning an i64 value explicitly
}

// Find the maximum value in a slice of i64 numbers
fn find_max(numbers: &[i64]) -> i64 {
    let max = Mutex::new(i64::MIN);
    numbers.par_iter().for_each(|&num| {
        let mut max_lock = max.lock().unwrap();
        if num > *max_lock {
            *max_lock = num;
        }
        // The lock is automatically dropped here as max_lock goes out of scope
    });
    // Explicitly create a new scope to ensure the lock is dropped before returning
    let max_value = *max.lock().unwrap();
    max_value // Now we are returning an i64 value explicitly
}

// Find the median value in a mutable slice of i64 numbers
fn find_median(numbers: &mut [i64]) -> i64 {
    numbers.par_sort_unstable(); // Parallel sorting for efficiency
    let mid = numbers.len() / 2;
    // Handle even and odd number of elements differently
    if numbers.len() % 2 == 0 {
        (numbers[mid - 1] + numbers[mid]) / 2
    } else {
        numbers[mid]
    }
}

// Find the mean value of a slice of i64 numbers
fn find_mean(numbers: &[i64]) -> f64 {
    let sum: i64 = numbers.par_iter().sum(); // Parallel summation for efficiency
    sum as f64 / numbers.len() as f64 // Calculate mean
}

// Find the longest increasing and decreasing subsequences in a slice of i64 numbers
fn find_longest_sequences(numbers: &[i64]) -> (Vec<i64>, Vec<i64>) {
    let mut longest_increasing = vec![];
    let mut current_increasing = vec![];

    let mut longest_decreasing = vec![];
    let mut current_decreasing = vec![];

    // Initialize with the first element if the slice is not empty
    if !numbers.is_empty() {
        current_increasing.push(numbers[0]);
        current_decreasing.push(numbers[0]);
    }

    for &num in numbers.iter().skip(1) {
        // Increasing sequence handling
        if num > *current_increasing.last().unwrap() {
            current_increasing.push(num);
        } else {
            if current_increasing.len() > longest_increasing.len() {
                longest_increasing = current_increasing.clone();
            }
            current_increasing.clear();
            current_increasing.push(num);
        }

        // Decreasing sequence handling
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

    // Check final sequences against the longest found
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
