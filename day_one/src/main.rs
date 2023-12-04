use std::fs;

fn main() {
    let input_string = fs::read_to_string("input.txt").unwrap();
    let input_iterator = input_string.split("\n");

    let mut accumulator = 0;

    input_iterator.for_each(|line| {
        let mut chars = line.chars();
        let mut first_char = chars.find(|char| {char.is_ascii_digit()}).unwrap().to_string();
        
        let mut reversed_chars = line.chars().rev();
        let last_char = reversed_chars.find(|char| {char.is_ascii_digit()}).unwrap().to_string();

        first_char.push_str(&last_char);
        let num = first_char.parse::<u32>().unwrap();
        accumulator += num;
    }
    );


    
    println!("{accumulator}");
}
