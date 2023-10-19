extern crate helpers;

use clap::Parser;
use helpers::{parse::parse, turtle::Turtle};
use unsvg::Image;
use std::{fs::read_to_string, collections::{HashMap, VecDeque}};
use helpers::conditions::*;

/// A simple program to parse four arguments using clap.
#[derive(Parser)]
struct Args {
    /// Path to a file
    file_path: std::path::PathBuf,

    /// Path to an svg or png image
    image_path: std::path::PathBuf,

    /// Height
    height: u32,

    /// Width
    width: u32,
}

fn main() -> Result<(), ()> {
    let args: Args = Args::parse();

    // Access the parsed arguments
    let file_path = args.file_path;
    let image_path = args.image_path;
    let height = args.height;
    let width = args.width;

    let mut image = Image::new(width, height);
    let mut turtle = Turtle::new(width, height);
    // store variables
    let mut variables: HashMap<String, f32> = HashMap::new();
    // store all while and ifs.
    let mut conditions: VecDeque<Condition> = VecDeque::new();

    // read in file
    let mut lines: Vec<&str> = Vec::new();
    let file_string: String;
    let result_to_string = read_to_string(file_path);
    match result_to_string {
        Ok(readed) => file_string = readed,
        Err(e) => {
            eprintln!("Error reading the lg file: {e}");
            return Err(());
        }
    }
    for line in file_string.lines() {
        lines.push(line);
    }
    let length = lines.len();
    let mut index: usize = 0;

    // split it into tokens and go line by line and modify the image.
    //for line in lines {     
    while  index < length { 
        let line = lines[index];
        let logoline = index + 1;
        println!("{line} line: {logoline}");
        let tokens = line.split_whitespace();
        let result = parse(tokens, &mut turtle, &mut image, &mut variables, &mut index, &mut conditions);
        if let Err(_e) = result {
            return Err(());
        }
    }    

    //after logo program is finished, if still in IF/WHILE bracket, report error
    if !conditions.is_empty() {
        println!("{conditions:?}");
        eprintln!("logo code ended within incomplete WHILE/IF bracket!");
        return Err(());
    }

    match image_path.extension().map(|s| s.to_str()).flatten() {
        Some("svg") => {
            let res = image.save_svg(&image_path);
            if let Err(e) = res {
                eprintln!("Error saving svg: {e}");
                return Err(());
            }
        }
        Some("png") => {
            let res = image.save_png(&image_path);
            if let Err(e) = res {
                eprintln!("Error saving png: {e}");
                return Err(());
            }
        }
        _ => {
            eprintln!("File extension not supported");
            return Err(());
        }
    }

    Ok(())
}