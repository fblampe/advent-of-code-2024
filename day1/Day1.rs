use std::collections::HashMap;
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
    
    println!("{}", calculate_total_distance(&list1, &list2));
    println!("{}", calculate_similarity_score(&list1, &list2));
}

fn calculate_total_distance(list1: &Vec<i32>, list2: &Vec<i32>) -> i32 {
    let sorted_pairs = list1.iter().zip(list2.iter());
    sorted_pairs.map(|(x, y)| (x - y).abs()).sum()
}

fn calculate_similarity_score(list1: &Vec<i32>, list2: &Vec<i32>) -> i32 {
    let mut counts: HashMap<i32, i32> = list2.iter().fold(HashMap::new(), |mut counts_acc, y| {
        counts_acc.entry(*y).and_modify(|y_count| *y_count += 1).or_insert(1);
        counts_acc
    });
    list1.iter().map(|x| x * *counts.entry(*x).or_insert(0)).sum()
}