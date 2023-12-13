use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
struct Part {
    row: usize,
    start_col: usize,
    end_col: usize,
    digits: usize,
}

fn parse_text(str_input: &str) -> (Vec<Part>, Vec<(usize, usize)>) {
    let mut current_digits: String = String::from("");
    let mut parts: Vec<Part> = Vec::new();
    let mut symbols: Vec<(usize, usize)> = Vec::new();

    str_input.lines().enumerate().for_each(|(row, line)| {
        line.chars().enumerate().for_each(|(col, ch)| {
            if ch.is_digit(10) {
                current_digits.push(ch)
            } else {
                if ch != '.' {
                    symbols.push((row, col));
                }
                if !current_digits.is_empty() {
                    parts.push(Part {
                        row,
                        start_col: col - current_digits.len(),
                        end_col: col - 1,
                        digits: current_digits.parse().unwrap(),
                    });
                    current_digits.drain(..);
                }
            }
        });

        if !current_digits.is_empty() {
            let col = line.len();
            println!("{row}, {col}, {:?}", current_digits);
            parts.push(Part {
                row,
                start_col: col - current_digits.len(),
                end_col: col - 1,
                digits: current_digits.parse().unwrap(),
            });
            current_digits.drain(..);
        }
    });

    (parts, symbols)
}

fn get_parts(str_input: &str) -> Vec<Part> {
    let mut current_digits: String = String::from("");
    let mut parts: Vec<Part> = Vec::new();

    let mut prev_symbols: HashSet<usize> = HashSet::new();
    let mut curr_symbols: HashSet<usize> = HashSet::new();

    let mut prev_parts: HashMap<usize, Vec<Part>> = HashMap::new();
    let mut curr_parts: HashMap<usize, Vec<Part>> = HashMap::new();

    let mut last_symbol_seen: Option<usize> = None;
    let mut last_part_seen: Option<Part> = None;

    str_input.lines().enumerate().for_each(|(row, line)| {
        line.chars().enumerate().for_each(|(col, ch)| {
            if ch.is_digit(10) {
                current_digits.push(ch);
            } else {
                if ch != '.' {
                    if col > 0 {
                        curr_symbols.insert(col - 1);
                    }
                    curr_symbols.insert(col);
                    curr_symbols.insert(col + 1);

                    if prev_parts.contains_key(&col) {
                        for p in prev_parts[&col] {
                            parts.push(p);
                        }
                    }
                }

                if !current_digits.is_empty() {
                    let current_part = Part {
                        row,
                        start_col: col - current_digits.len(),
                        end_col: col - 1,
                        digits: current_digits.parse().unwrap(),
                    };

                    if prev_symbols.contains(&col) {
                        parts.push(current_part.clone());
                    }

                    if col > 0 {
                        curr_parts.entry(col - 1).or_insert(Vec::new());
                        curr_parts[&(col - 1)].push(current_part.clone());
                    }

                    curr_parts.entry(col).or_insert(Vec::new());
                    curr_parts[&col].push(current_part.clone());

                    curr_parts.entry(col + 1).or_insert(Vec::new());
                    curr_parts[&(col + 1)].push(current_part.clone());

                    current_digits.drain(..);
                }
            }
        });

        if !current_digits.is_empty() {
            let col = line.len();
            println!("{row}, {col}, {:?}", current_digits);
            parts.push(Part {
                row,
                start_col: col - current_digits.len(),
                end_col: col - 1,
                digits: current_digits.parse().unwrap(),
            });
            current_digits.drain(..);
        }

        prev_symbols = curr_symbols.clone();
        curr_symbols.drain();

        prev_parts = curr_parts.clone();
        curr_parts.drain();
    });

    parts
}

fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let (parts, symbols) = parse_text(&input);
    // println!("{:?}, {:?}", parts, symbols);
}
