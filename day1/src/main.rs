use std::fs;


fn count_increased_values(values: &Vec<u32>) -> u32 {
    let mut increased_count = 0;
    let mut last_val: u32 = 0;

    // Part 1
    for (i, &val) in values.iter().enumerate() {
        let mut tendency: &str = "nop";

        if i != 0 {
            if val > last_val {
                tendency = "increased";
                increased_count += 1;
            }
            else {
                tendency = "decreased";
            }
        }
        
        println!("{} ({})", val, tendency);
        last_val = val;
    }
    increased_count
}



fn main() {
    const MEASURES_FILENAME: &str = "resources/measures";

    let content = fs::read_to_string(MEASURES_FILENAME).unwrap();
    
    // Part 1
    let measures: Vec<u32> = content.lines().map(|x| x.parse::<u32>().unwrap()).collect();
    let part1_answer = count_increased_values(&measures);
    println!("Part1 answer: {} times increased", part1_answer);

    // Part 2
    let mut measures_windowed = vec!();

    let windows = measures.windows(3);
    for window in windows {
        measures_windowed.push(window.iter().sum());
    }
    
    let part2_answer = count_increased_values(&measures_windowed);
    println!("Part2 answer: {} times increased", part2_answer);
}