use std::fs;
use std::collections::HashMap;


#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue
}

impl Color {
    fn parse(color_str: &str) -> Self {
        match color {
            "red" => Self::Red,
            "green" => Self::Green,
            "blue" => Self::Blue,
            _ => panic!("Unknown color! Panic!")
        }
    }
}

#[derive(Debug)]
struct Cube {
    number: usize,
    color: Color,
}

impl Cube {
    fn parse(cube_string: &str) -> Self {
        let (number_string, color_string) = cube.trim().split_once(" ").unwrap();
        let number = &number_string.trim().parse::<usize>().unwrap();
        let color = Color::parse(color_string);

        Self {
            number,
            color
        }
    }
}

#[derive(Debug)]
struct Game {
    cubes: Vec<Cube>
}

impl Game {
    fn parse(game_string: &str) -> Self {
        let (_, second_half) = game_string.split_once(":").unwrap();

       let cubes = second_half.trim().split(|ch| ch == ';' || ch == ',').map(|str| {
        Cube::parse(str)
       }).collect();

       Self {
        cubes
       }
    }
}

fn main() {
    let input_string= fs::read_to_string("input.txt").unwrap();
    let input_iterator = input_string.split("\n");

    input_iterator.map(|line| {
        let game = Game::parse(line);
        println!("{:?}", game)
    })

    // let cube_only_strings = input_iterator.map(|line| {
    //     let (_, second_half) = line.split_once(":").unwrap();
    //    second_half.trim()
    //    // ^this is an automatic return
    // }).collect::<Vec<_>>();

    

    let mut power_accumulator = 0;

    // let cubes_holder: usize = cube_only_strings.iter().enumerate().map(|(i, str)| {
    //     println!("-----new game-----");
    //     println!("{}", str);
    //     let mut cubes = str.split(|ch| ch == ';' || ch == ',');
    //     let mut largest_cubes = HashMap::new();
    //     largest_cubes.insert(String::from("green"), 1);
    //     largest_cubes.insert(String::from("blue"), 1);
    //     largest_cubes.insert(String::from("red"), 1);

    //     let legal_game = cubes.all(|cube| {
    //         let (number, color) = cube.trim().split_once(" ").unwrap();
    //         println!("color: {}", color);
    //         let number_of_color = &number.trim().parse::<usize>().unwrap();
    //         let number_of_color_result = &number.trim().parse::<usize>();
            
    //         compare_sizes(&mut largest_cubes, number_of_color, color);

    //         match number_of_color_result {
    //             Ok(number_of_color) => number_of_color <= &color_size(color),
    //             Err(error) => panic!("Not a number: {:?} at line {:?}, {:?}", error, i + 1, cube)
    //         }
    //     });

    //     let power = largest_cubes.get("green").unwrap() * largest_cubes.get("blue").unwrap() * largest_cubes.get("red").unwrap();
    //     power_accumulator = power_accumulator + power;
    //     println!("power: {}", power);
    //     println!("accumulator: {}", power_accumulator);
    //     dbg!(largest_cubes);

    //     if legal_game {
    //         i + 1         
    //     } else {
    //         0
    //     }
    // }).sum();

    dbg!(power_accumulator);
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

fn compare_sizes(largest: &mut HashMap<String, usize>, number: &usize, color: &str) {
    // dbg!(&largest);
    println!("color: {} and number: {}", color, number);
    if largest.get(color).unwrap() < number {
        largest.insert(color.to_string(), *number);
    }
}

static buggy_string: &str = "19 red, 8 green, 9 blue; 7 blue, 1 red, 9 green; 2 red, 9 blue, 11 green; 1 blue, 4 green, 10 red; 10 blue, 11 red; 4 green, 8 blue, 16 red";