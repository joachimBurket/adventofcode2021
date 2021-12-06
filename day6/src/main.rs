use std::fs;
mod lantern_fish;
use lantern_fish::LanternFish;

const GESTATION_TIME: u32 = 7;

fn part1(content: &str, experiment_duration: u32) -> usize {
    let mut fishes: Vec<LanternFish> = Vec::new();
    let fishes_timers: Vec<u32> = content.split(",").map(|x| x.parse::<u32>().unwrap()).collect();

    for timer in fishes_timers {
        fishes.push(LanternFish::new(timer, GESTATION_TIME));
    }

    for _ in 1..=experiment_duration {
        let mut new_fishes: Vec<LanternFish> = Vec::new();
        for fish in &mut fishes {
            if fish.age_one_day() {
                new_fishes.push(LanternFish::new(GESTATION_TIME+1, GESTATION_TIME));
            }
        }
        fishes.append(&mut new_fishes);
    }

    fishes.len()
}

fn part2(content: &str, experiment_duration: u32) -> u128 {
    let fishes: Vec<u128> = content.split(",").map(|x| x.parse::<u128>().unwrap()).collect();
    let mut indexes: Vec<u128> = vec![0; (GESTATION_TIME+2) as usize];
    let mut new_indexes: Vec<u128> = vec![0; (GESTATION_TIME+2) as usize];

    // fills the indexes array
    for fish in fishes {
        indexes[fish as usize] += 1;
    }

    for _ in 1..=experiment_duration {
        for i in (0..indexes.len()).rev() {
            if i == 0 {
                new_indexes[6] += indexes[i];
                new_indexes[8] = indexes[i];
            }
            else {
                new_indexes[i-1] = indexes[i];
            }
        }
        indexes = new_indexes.clone();
    }
    indexes.into_iter().sum()
}

fn main() {
    const INPUT_FILENAME: &str = "resources/lantern_fishes";
    

    // Part 1
    let content = fs::read_to_string(INPUT_FILENAME).unwrap();
    let experiment_duration = 80;
    let part1_answer = part1(&content, experiment_duration);
    println!("Part1 answer: {} fishes after {} days of experiment", part1_answer, experiment_duration);

    // Part 2
    let experiment_duration = 256;
    let part2_answer = part2(&content, experiment_duration);
    println!("Part2 answer: {} fishes after {} days of experiment", part2_answer, experiment_duration);    

}
