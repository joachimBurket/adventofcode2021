use std::fs;

fn main() {
    const INPUT_FILENAME: &str = "resources/crabs";

    let content = fs::read_to_string(INPUT_FILENAME).unwrap();

    let mut crabs: Vec<u32> = content.split(",").map(|x| x.parse::<u32>().unwrap()).collect();
    crabs.sort();

    // Part 1
    let median: u32 = crabs[crabs.len() / 2.0 as usize];
    println!("median = {}", median);

    let fuels: Vec<u32> = vec![median; crabs.len()].iter().zip(crabs.iter()).map(|(&b, &v)| (b as i32 - v as i32).abs() as u32).collect();
    let total_fuel: u32 = fuels.iter().sum();
    println!("Part1 answer: Best position: {}, min: {}", median, total_fuel);


    // Part2
    let sum: u32 = crabs.iter().sum();
    let mean: f32 = sum as f32 / crabs.len() as f32;
    let best_index: u32 = mean.floor() as u32;
    println!("mean = {}, best_index = {}", mean, best_index);

    let mut fuels: Vec<u32> = Vec::new();
    for crab in crabs {
        let diff = (crab as i32 - best_index as i32).abs();
        let fuel = (diff * (diff+1)) as f32 / 2.0;
        fuels.push(fuel as u32);
    }
    let total_fuel: u32 = fuels.iter().sum();
    println!("Part2 answer: Best position: {}, min: {}", best_index, total_fuel);
}
