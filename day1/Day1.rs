use std::fs;
use std::str::FromStr;

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let all_numbers: Vec<_> = input.split_whitespace().map(| s | i32::from_str(s).unwrap()).collect();
    
    let mut list1: Vec<i32> = vec![];
    let mut list2: Vec<i32> = vec![];
    all_numbers.chunks(2).for_each(|pair| {
        list1.push(*pair.first().unwrap()); 
        list2.push(*pair.last().unwrap());
    });

    list1.sort();
    list2.sort();
    let sorted_pairs = list1.iter().zip(list2.iter());
    let result: i32 = sorted_pairs.map(|(x, y)| (x - y).abs()).sum();

    println!("{}", result);
}