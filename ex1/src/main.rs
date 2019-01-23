// Given a list of integers, use a vector and return the mean
// (the average value), median (when sorted, the value in the
//  middle position), and mode (the value that occurs most
// often; a hash map will be helpful here) of the list.

use std::collections::HashMap;

fn main() {
    let mut list = vec![2, 3, 4, 5, 6, 7, 8, 1, 3, 3];
    println!("{:?}", list);
    let avg = average(&list);
    let median = median(&mut list);
    let mode = mode(&list);
    println!("{:?}", list);
    println!("{:?}", avg);
    println!("{:?}", median);
    println!("{:?}", mode);
}

fn average(slice: &[i32]) -> f32 {
    let len = slice.len();
    if len == 0 {
        0.0
    } else {
        slice.iter().sum::<i32>() as f32 / len as f32
    }
}

fn median(slice: &mut [i32]) -> i32 {
    slice.sort();
    let mid = slice.len() / 2;
    slice[mid]
}

fn mode(slice: &[i32]) -> i32 {
    let mut map = HashMap::new();

    for &number in slice {
        let count = map.entry(number).or_insert(0);
        *count += 1;
    }

    get_mode(&map)
}

fn get_mode(map: &HashMap<i32, i32>) -> i32 {
    let mut number = 0;
    let mut occurence = 0;
    for (&n, &o) in map{
        if o > occurence { number = n; occurence = o; }
    }
    number
}
