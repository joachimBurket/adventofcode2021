use std::fs;
use ndarray::prelude::*;
use ndarray::Array;

type Point = ((usize,usize), u32);

struct Basin {
    pub points: Vec<Point>
}

impl Basin {
    pub fn new(init_vec: Vec<Point>) -> Basin {
        Basin {
            points: init_vec,
        }
    }
}

fn parse_input(content: &str) -> Array2<u32> {
    let rows = content.lines().count();
    let cols: usize = content.lines().next().unwrap().chars().collect::<Vec<char>>().len();
    let mat_vec = content.lines()
        .map(|x| x.chars().map(|x| x.to_digit(10).unwrap()).collect::<Vec<u32>>())
        .flatten()
        .collect::<Vec<u32>>();

    Array::from_shape_vec((rows, cols), mat_vec).unwrap()
}

/// Returning low points in a vector of ((y,x), val)
fn find_low_points(mat: &Array2<u32>) -> Vec<Point> {
    let mut low_points: Vec<((usize,usize),u32)> = Vec::new();

    for ((y, x), elt) in mat.indexed_iter() {
        let left = if x > 0 { mat[(y, x-1)] } else { u32::MAX };
        let right = if x < mat.ncols()-1 { mat[(y, x+1)] } else {u32::MAX };
        let up = if y > 0 { mat[(y-1, x)] } else { u32::MAX };
        let down = if y < mat.nrows()-1 { mat[(y+1, x)] } else { u32::MAX };

        if *elt < left && *elt < right && *elt < up && *elt < down {
            low_points.push(((y,x), *elt));
        }
    }
    low_points
}

fn find_basins(mat: &Array2<u32>, low_points: &Vec<((usize,usize),u32)>) -> Vec<Basin> {
    let mut basins: Vec<Basin> = Vec::new();

    for low_point in low_points {
        let basin = locate_basin(mat, *low_point);
        basins.push(basin);
    }

    basins
}

/// Locating the Basin frontiers using Breadth first search
fn locate_basin(mat: &Array2<u32>, start_point: ((usize,usize),u32)) -> Basin {
    let mut basin = Basin::new(vec![start_point]);
    let mut frontier: Vec<((usize,usize),u32)> = Vec::new();
    let mut reached: Array2<bool> = Array2::from_elem((mat.nrows(), mat.ncols()), false);

    frontier.push(start_point);
    let (y,x) = start_point.0;
    reached[(y,x)] = true;
    
    while !frontier.is_empty() {
        let current = frontier.pop().unwrap();
        
        for next in get_neighbors(mat, current) {
            let (y,x) = next.0;
            if !reached[(y,x)] {
                frontier.push(next);
                reached[(y,x)] = true;
                basin.points.push(next);
            }
        }
    }
    basin
}

fn get_neighbors(mat: &Array2<u32>, point: ((usize,usize),u32)) -> Vec<((usize,usize),u32)> {
    let mut neighbors: Vec<((usize,usize),u32)> = Vec::new();
    let (y,x) = point.0;

    if x > 0 { // left
        neighbors.push(((y,x-1), mat[(y,x-1)]));
    }
    if x < mat.ncols()-1 {  // right
        neighbors.push(((y,x+1), mat[(y,x+1)]));
    }
    if y > 0 {  // up
        neighbors.push(((y-1, x), mat[(y-1, x)]));
    }
    if y < mat.nrows()-1 { // down
        neighbors.push(((y+1, x), mat[(y+1, x)]));
    }

    neighbors.into_iter().filter(|x| x.1 != 9).collect()
}

fn compute_risk_level(low_points: &Vec<u32>) -> u32 {
    low_points.iter().map(|x| x+1).sum()
}

fn main() {
    const INPUT_FILENAME: &str = "resources/smoke_heightmap";
    let content = fs::read_to_string(INPUT_FILENAME).unwrap();
    let mat = parse_input(&content);

    let low_points = find_low_points(&mat);
    let risk_level = compute_risk_level(&low_points.iter().map(|x| x.1).collect());
    println!("Low points: {:?}", low_points);
    println!("Part1 answer: Risk level = {}", risk_level);

    let basins = find_basins(&mat, &low_points);
    let mut basins_sizes: Vec<usize> = basins.iter().map(|x| x.points.len()).collect();
    basins_sizes.sort();
    let largests_sizes_mult: usize = basins_sizes.iter().rev().take(3).fold(1, |acc,x| acc * x);
    println!("Part2 answer: 3 largest basins sizes multiplication = {}", largests_sizes_mult);
}
