use std::fs::File;
use std::io::prelude::*;

struct Grid {
    data: Vec<Vec<Option<u8>>>,
}

fn read_grid(filename: &str) -> std::io::Result<Grid> {
    let mut file = File::open(filename)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    content.pop();

    Ok(Grid {
        data: content
            .split("\n")
            .map(|line| {
                line.chars()
                    .map(|c| {
                        if c == '_' {
                            None
                        } else {
                            Some(c.to_digit(10)? as u8)
                        }
                    })
                    .collect()
            })
            .collect(),
    })
}

fn value_to_string(value: &Option<u8>) -> String {
    value.map(|v| v.to_string()).unwrap_or(' '.to_string())
}

fn line_to_string(line: &Vec<Option<u8>>) -> String {
    let vec: Vec<String> = line.iter().map(|v| value_to_string(v)).collect();
    ["│ ", &vec.join(" │ "), " │"].concat()
}

const SEP_NORTH:  &'static str =   "┌───┬───┬───┬───┬───┬───┬───┬───┬───┐\n";
const SEP_MIDDLE: &'static str = "\n├───┼───┼───┼───┼───┼───┼───┼───┼───┤\n";
const SEP_SOUTH:  &'static str = "\n└───┴───┴───┴───┴───┴───┴───┴───┴───┘";

fn grid_to_string(grid: &Grid) -> String {
    let lines: Vec<String> = grid.data.iter().map(|line| line_to_string(&line)).collect();
    [SEP_NORTH, &lines.join(SEP_MIDDLE), SEP_SOUTH].concat()
}

fn print_grid(grid: &Grid) {
    println!("{}", grid_to_string(&grid));
}

fn main() {
    let grid = read_grid("data.txt").unwrap();
    print_grid(&grid)
}