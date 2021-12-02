use std::fs;

fn navigate1(navigation_instructions: &str) -> u32 {
    let mut pos = (0,0);

    for instr in navigation_instructions.lines() {
        let mut split = instr.split_ascii_whitespace();
        let direction = split.next().unwrap();
        let distance = split.next().unwrap().parse::<u32>().unwrap();

        match direction {
            "forward" =>  pos.0 += distance,
            "up" =>  pos.1 -= distance,
            "down" =>  pos.1 += distance,
            _ => panic!("Don't know this direction!"),
        };
    }
    println!("Part1: Final position: {:?}", pos);
    pos.0 * pos.1
}

fn navigate2(navigation_instructions: &str) -> u32 {
    let mut pos = (0, 0);
    let mut aim = 0;

    for instr in navigation_instructions.lines() {
        let mut split = instr.split_ascii_whitespace();
        let direction = split.next().unwrap();
        let distance = split.next().unwrap().parse::<u32>().unwrap();

        match direction {
            "forward" =>  {
                pos.0 = pos.0 + distance;
                pos.1 = pos.1 + aim * distance;
            },
            "up" =>  aim -= distance,
            "down" =>  aim += distance,
            _ => panic!("Don't know this direction!"),
        };
    }
    println!("Part2: Final position: {:?}, Aim: {}", pos, aim);
    pos.0 * pos.1
}

fn main() {
    const INPUT_FILENAME: &str = "resources/planned_course";

    let content = fs::read_to_string(INPUT_FILENAME).unwrap();
    
    // Part 1
    let part1_answer = navigate1(&content);
    println!("Part1: Multiplication: {}\n", part1_answer);

    // Part 2
    let part2_answer = navigate2(&content);
    println!("Part2: Multiplication: {}\n", part2_answer);

}