use std::fs::File;
use std::io::prelude::*;

struct Grid {
    data: Vec<Vec<Option<u8>>>,
}

fn read_value(c: char) -> Option<u8> {
    if c == '_' || c == '0' {
        None
    } else {
        Some(c.to_digit(10)? as u8)
    }
}

fn read_line(line: &str) -> Vec<Option<u8>> {
    line.chars().map(|c| read_value(c)).collect()
}

fn read_grid(filename: &str) -> std::io::Result<Grid> {
    let mut file = File::open(filename)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    content.pop();

    Ok(Grid {
        data: content.split("\n").map(|line| read_line(line)).collect(),
    })
}

fn value_to_string(value: &Option<u8>) -> String {
    value.map(|v| v.to_string()).unwrap_or(' '.to_string())
}

fn line_to_string(line: &Vec<Option<u8>>) -> String {
    let vec: Vec<String> = line.iter().map(|v| value_to_string(v)).collect();
    [
        "│ ",
        &vec.chunks(3)
            .map(|chunk| chunk.join(" │ "))
            .collect::<Vec<_>>()
            .join(" ║ "),
        " │",
    ].concat()
}

const SEP_NORTH: &'static str =   "┌───┬───┬───╥───┬───┬───╥───┬───┬───┐\n";
const SEP_MINOR: &'static str = "\n├───┼───┼───╫───┼───┼───╫───┼───┼───┤\n";
const SEP_MAJOR: &'static str = "\n╞═══╪═══╪═══╬═══╪═══╪═══╬═══╪═══╪═══╡\n";
const SEP_SOUTH: &'static str = "\n└───┴───┴───╨───┴───┴───╨───┴───┴───┘";

fn grid_to_string(grid: &Grid) -> String {
    let lines: Vec<String> = grid.data.iter().map(|line| line_to_string(&line)).collect();
    [
        SEP_NORTH,
        &lines
            .chunks(3)
            .map(|chunk| chunk.join(SEP_MINOR))
            .collect::<Vec<_>>()
            .join(SEP_MAJOR),
        SEP_SOUTH,
    ].concat()
}

fn print_grid(grid: &Grid) {
    println!("{}", grid_to_string(&grid));
}

fn main() {
    let grid = read_grid("data.txt").unwrap();
    print_grid(&grid)
}
