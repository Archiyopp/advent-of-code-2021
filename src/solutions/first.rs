use std::collections::VecDeque;

pub fn first_half_puzzle() {
    let input = advent_of_code::read_file("inputs", "first");
    let mut depth: Option<i32> = None;

    let counter = input
        .lines()
        .map(|line| line.trim().parse::<i32>().expect("input to be numbers"))
        .fold(0, |mut counter, new_depth| {
            if let Some(depth_value) = depth {
                if new_depth > depth_value {
                    counter += 1;
                }
            }
            depth = Some(new_depth);
            counter
        });
    println!(
        "Day 1: First half: number of depth increments were {}",
        counter
    );
}
pub fn second_half_puzzle() {
    let input = advent_of_code::read_file("inputs", "first");
    let mut depths_array = VecDeque::from([None; 3]);
    let mut depth_sum = i32::MAX;
    let mut counter = 0;
    for line in input.lines() {
        let new_depth: i32 = match line.trim().parse() {
            Ok(value) => value,
            Err(_) => break,
        };
        let last_value_is_some = depths_array
            .pop_front()
            .expect("Expected to get first option")
            .is_some();
        depths_array.push_back(Some(new_depth));

        let current_array: Vec<i32> = depths_array.iter().filter_map(|v| *v).collect();
        let new_sum: i32 = current_array.iter().sum();
        if last_value_is_some && current_array.len() == 3 && new_sum > depth_sum {
            counter += 1;
        }
        depth_sum = new_sum;
    }
    println!(
        "Day 1: Second half: number of depth increments were {}",
        counter
    );
}
