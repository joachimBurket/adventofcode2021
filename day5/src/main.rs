use std::fs;
use itertools::Itertools;
use ndarray::prelude::*;
mod lib;
use lib::{Segment, Point};


fn part1(content: &str) -> usize {
    let mut segments: Vec<Segment> = Vec::new();
    let mut coord_values: Vec<usize> = Vec::new();  // stores all the coordinates values

    for line in content.lines() {
        let points: (&str, &str) = line.splitn(2, " -> ").collect_tuple().unwrap();
        let p1: (usize, usize) = points.0.splitn(2, ",").map(|x| x.parse::<usize>().unwrap()).collect_tuple().unwrap();
        let p2: (usize, usize) = points.1.splitn(2, ",").map(|x| x.parse::<usize>().unwrap()).collect_tuple().unwrap();
        segments.push(Segment::new((Point::new(p1), Point::new(p2))));
        coord_values.append(&mut Vec::from([p1.0, p1.1, p2.0, p2.1]));
    }

    // filters horizontal and vertical segments
    let segments: Vec<Segment> = segments.into_iter()
        .filter(|x| x.is_horizontal() || x.is_vertical())
        .collect();

    let max_coord_val = coord_values.iter().max().unwrap();
    let mut map = Array2::<u32>::zeros(((max_coord_val+1) as usize, (max_coord_val+1) as usize));

    for segment in segments {   
        for point in segment.list_points() {
            map[[point.y, point.x]] += 1;
        }
    }
    println!("Map:\n{:?}", map);

    map.iter().filter(|x| **x > 1).count()
}

fn part2(content: &str) -> usize {
    // TODO: miss 45 degrees points listing
    let mut segments: Vec<Segment> = Vec::new();
    let mut coord_values: Vec<usize> = Vec::new();  // stores all the coordinates values

    for line in content.lines() {
        let points: (&str, &str) = line.splitn(2, " -> ").collect_tuple().unwrap();
        let p1: (usize, usize) = points.0.splitn(2, ",").map(|x| x.parse::<usize>().unwrap()).collect_tuple().unwrap();
        let p2: (usize, usize) = points.1.splitn(2, ",").map(|x| x.parse::<usize>().unwrap()).collect_tuple().unwrap();
        segments.push(Segment::new((Point::new(p1), Point::new(p2))));
        coord_values.append(&mut Vec::from([p1.0, p1.1, p2.0, p2.1]));
    }

    // filters horizontal and vertical segments
    let segments: Vec<Segment> = segments.into_iter()
        .filter(|x| x.is_horizontal() || x.is_vertical() || x.is_45_degrees())
        .collect();
    
    let max_coord_val = coord_values.iter().max().unwrap();
    let mut map = Array2::<u32>::zeros(((max_coord_val+1) as usize, (max_coord_val+1) as usize));

    for segment in segments {   
        for point in segment.list_points() {
            map[[point.y, point.x]] += 1;
        }
    }
    println!("Map:\n{:?}", map);
    
    map.iter().filter(|x| **x > 1).count()
}


fn main() {
    const INPUT_FILENAME: &str = "resources/hydrothermal_vents";

    let content = fs::read_to_string(INPUT_FILENAME).unwrap();

    // Part1
    let part1_answer = part1(&content);
    println!("Part1 answer: sum = {}", part1_answer);

    // Part2
    let part2_answer = part2(&content);
    //println!("Part2 answer: sum = {}", part2_answer);
}

