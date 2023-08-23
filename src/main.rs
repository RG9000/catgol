use rand::Rng;
use std::{thread, time};

fn main() {
    if let Some(pipe) = read_pipe::read_pipe() {
        if let Some((t_width, t_height)) = term_size::dimensions() {
            let input: Vec<Vec<char>> = convert_input_to_array(&pipe, t_width - 2, t_height - 5);
            let mut output = input;
            let possible_chars: Vec<char> = get_possible_characters(&pipe);
            loop {
                thread::sleep(time::Duration::from_millis(500));
                print!("{}[2J", 27 as char);
                print_output(&output);
                output = get_next_output(&output, &possible_chars);
            }
        } else {
            eprintln!("error: unable to get term size");
        }
    } else {
        eprintln!("error: no text supplied");
    }
}

fn get_next_output(input: &Vec<Vec<char>>, possible_chars: &Vec<char>) -> Vec<Vec<char>> {
    let mut output: Vec<Vec<char>> = vec![vec![' '; input[0].len()]; input.len()];
    let mut rng: rand::rngs::ThreadRng = rand::thread_rng();

    for (y, l) in input.iter().enumerate() {
        for (x, c) in l.iter().enumerate() {
            if y > input.len() - 2 || x > input[0].len() - 2 || x == 0 || y == 0 {
                continue;
            }
            let mut neighbours = 0;
            for y2 in (y - 1)..=(y + 1) {
                for x2 in (x - 1)..=(x + 1) {
                    if !input[y2][x2].is_whitespace() {
                        neighbours += 1;
                    }
                }
            }

            //dead
            if c.is_whitespace() {
                if neighbours == 3 {
                    let n: usize = rng.gen_range(0..possible_chars.len());
                    output[y][x] = possible_chars[n];
                }
            }
            //alive
            else {
                if neighbours == 3 || neighbours == 4 {
                    output[y][x] = *c;
                }
            }
        }
    }
    return output;
}

fn print_output(input: &Vec<Vec<char>>) {
    for l in input {
        for c in l {
            if c.is_whitespace() {
                print!(" ")
            } else {
                print!("{}", c);
            }
        }
        print!("\n");
    }
}

fn get_possible_characters(input: &String) -> Vec<char> {
    let mut chars: Vec<char> = Vec::new();
    for c in input.chars() {
        if !chars.contains(&c) && !c.is_whitespace() && c != '\n' {
            chars.push(c);
        }
    }
    return chars;
}

fn convert_input_to_array(input: &String, w: usize, h: usize) -> Vec<Vec<char>> {
    let mut arr: Vec<Vec<char>> = vec![vec![' '; w]; h];
    let mut x: usize = 0;
    let mut y: usize = 0;

    for c in input.chars() {
        if x >= w || y >= h {
            break;
        }
        if c == '\n' || x >= w {
            x = 0;
            y += 1;
            continue;
        }
        arr[y][x] = c;
        x += 1;
    }
    return arr;
}
