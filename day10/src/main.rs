#[macro_use]
extern crate lazy_static;

use std::fs;
use std::collections::HashMap;

lazy_static! {
    static ref OPEN_CHARS: HashMap<char, char> = {
        let mut m = HashMap::new();
        m.insert('(', ')');
        m.insert('{', '}');
        m.insert('[', ']');
        m.insert('<', '>');
        m
    };
}

lazy_static! {
    static ref CLOSE_CHARS: HashMap<char, char> = {
        let mut m = HashMap::new();
        m.insert(')', '(');
        m.insert('}', '{');
        m.insert(']', '[');
        m.insert('>', '<');
        m
    };
}

/// Checking corrupted lines.
/// Removing corrupted lines from the input and returning the
/// corrupted chars in a vector
fn check_corrupted_lines(lines: Vec<&str>) -> (Vec<&str>, Vec<char>) {
    let mut corrupted_chars: Vec<char> = Vec::new();
    let mut not_corrupted_lines: Vec<&str> = Vec::new();

    for line in lines {
        let mut opened: Vec<char> = Vec::new();
        let mut corrupted = false;

        for chr in line.chars() {
            if OPEN_CHARS.contains_key(&chr) {
                opened.push(chr);
            }
            else if CLOSE_CHARS.contains_key(&chr) {
                let op: char = opened.pop().unwrap_or('$');
                if *CLOSE_CHARS.get(&chr).unwrap() != op {
                        corrupted_chars.push(chr);
                        corrupted = true;
                        break;
                }
            }
            else {
                panic!("Char '{}' not recognized!", chr);
            }
        }
        if !corrupted {
            not_corrupted_lines.push(line);
        }
    }
    (not_corrupted_lines, corrupted_chars)
}

fn compute_syntax_checker_score(corrupted_chars: &Vec<char>) -> u32 {
    let mut close_chars_value = HashMap::new();
    close_chars_value.insert(')', 3);
    close_chars_value.insert(']', 57);
    close_chars_value.insert('}', 1197);
    close_chars_value.insert('>', 25137);

    let mut score: u32 = 0;

    for chr in corrupted_chars {
        score += close_chars_value.get(chr).unwrap();
    }
    score
}

fn do_the_autocomplete(lines: &Vec<&str>) -> Vec<Vec<char>> {
    let mut autocompletes: Vec<Vec<char>> = Vec::new();

    for line in lines {
        let mut opened: Vec<char> = Vec::new();
        
        for chr in line.chars() {
            if OPEN_CHARS.contains_key(&chr) {
                opened.push(chr);
            }
            else {
                opened.pop();
            }
        }

        let autocomplete: Vec<char> = opened.iter().map(|x| *OPEN_CHARS.get(&x).unwrap()).rev().collect();
        autocompletes.push(autocomplete);
    }

    autocompletes
}

fn compute_autocomplete_score(completing_chars: &Vec<Vec<char>>) -> u128 {
    let mut close_chars_value = HashMap::new();
    close_chars_value.insert(')', 1);
    close_chars_value.insert(']', 2);
    close_chars_value.insert('}', 3);
    close_chars_value.insert('>', 4);

    let mut scores: Vec<u128> = Vec::new();
    
    for line in completing_chars {
        let mut score: u128 = 0;
        for chr in line {
            score *= 5;
            score += close_chars_value.get(&chr).unwrap();
        }
        scores.push(score);
    }
    scores.sort();
    scores[(scores.len() as f32/2.0) as usize]
}


fn main() {
    const INPUT_FILENAME: &str = "resources/navigation_subsystem";
    let content = fs::read_to_string(INPUT_FILENAME).unwrap();
    let lines: Vec<&str> = content.lines().collect();

    let (lines, corrupted_chars) = check_corrupted_lines(lines);
    println!("Corrupted chars: {:?}", corrupted_chars);
    let syntax_checker_score = compute_syntax_checker_score(&corrupted_chars);
    println!("Part1 answer: Syntax checker score is {} points!", syntax_checker_score);

    let autocompletes = do_the_autocomplete(&lines);
    let autocomplete_score = compute_autocomplete_score(&autocompletes);
    println!("Part2 answer: Autocomplete score is {} points!", autocomplete_score);
}
