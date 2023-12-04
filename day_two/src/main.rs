use std::fs;

fn main() {
    let input_string= fs::read_to_string("input.txt").unwrap();
    let input_iterator = input_string.split("\n");

    let cube_only_strings = input_iterator.map(|line| {
        let (_, second_half) = line.split_once(":").unwrap();
       second_half.trim()
       // ^this is an automatic return
    }).collect::<Vec<_>>();

    let cubes_holder: usize = cube_only_strings.iter().enumerate().map(|(i, str)| {
        let mut cubes = str.split(|ch| ch == ';' || ch == ',');
        let legal_game = cubes.all(|cube| {
            let (number, color) = cube.trim().split_once(" ").unwrap();
            let number_of_color = number.trim().parse::<usize>();
            match number_of_color {
                Ok(number_of_color) => number_of_color <= color_size(color),
                Err(error) => panic!("Not a number: {:?} at line {:?}, {:?}", error, i + 1, cube)
            }
        });
        if legal_game {
            i + 1
        } else {
            0
        }
    }).sum();

    dbg!(cubes_holder);
}

fn color_size(color: &str) -> usize {
    //^ the & notates that it's pass by reference
    //usize - size matches the size of a # on the computer running the program
    match color {
        "red" => 12,
        "green" => 13,
        "blue" => 14,
        _ => panic!("Unknown color! Panic!")
    }
}