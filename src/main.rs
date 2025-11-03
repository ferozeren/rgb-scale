use clap::Parser;
use colored::Colorize;
use std::{fmt::Display, num::ParseIntError};

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[arg(short, long, default_value_t = 1.0)]
    scale: f32,
    /// RGB Value (e.g., "255,128,64") | Hex Code (e.g., ff3342)
    rgb: Option<String>,
    // #[arg(short, long, default_value_t = 0.1)]
    // version: f32,
}

#[derive(Debug)]
enum ValueError {
    Parse(ParseIntError),
}

impl Display for ValueError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let description: String = match self {
            ValueError::Parse(e) => format!("Invalid args, error: {}.", e),
        };
        f.write_str(&description)
    }
}

fn is_valid_hex(hex: &str) -> bool {
    hex.chars().all(|c| c.is_ascii_hexdigit())
}

fn convert_to_numeric(color_value: &[&str]) -> Result<Vec<u8>, ValueError> {
    let mut numeric_vec: Vec<u8> = Vec::new();
    for idx in color_value {
        let digit: u8 = idx.parse().map_err(|e| ValueError::Parse(e))?;
        numeric_vec.push(digit);
    }
    Ok(numeric_vec)
}

fn scale(original: &[u8], scale: f32) -> Vec<u8> {
    let mut output: Vec<u8> = Vec::new();
    for &i in original {
        let multiply: f32 = i as f32 * scale;
        output.push(multiply as u8);
    }
    output
}

fn print_to_console(value: &[u8], scaled_values: &[u8], scale: f32) {
    print!(
        "{color_sample}",
        color_sample = "\n■■ ".truecolor(value[0], value[1], value[2])
    );
    println!(
        "Original Value:\n{},{},{} | {:02x}{:02x}{:02x}\n",
        value[0], value[1], value[2], value[0], value[1], value[2],
    );
    print!(
        "{color_sample}",
        color_sample = "■■ ".truecolor(scaled_values[0], scaled_values[1], scaled_values[2])
    );
    println!(
        "Scaled Value ({}):\n{},{},{} | {:02x}{:02x}{:02x}",
        scale,
        scaled_values[0],
        scaled_values[1],
        scaled_values[2],
        scaled_values[0],
        scaled_values[1],
        scaled_values[2]
    );
}

fn handle_hex(hex: &str, cli_scale: f32) {
    let mut result_vec: Vec<u8> = Vec::new();
    if is_valid_hex(hex) {
        if hex.len() < 5 || hex.len() > 6 {
            println!("Invalid hex code, see -h for help");
            std::process::exit(0);
        }
        let vec_char: Vec<char> = hex.chars().collect();

        for char_slice in vec_char.chunks(2) {
            let width: String = char_slice.iter().collect();
            let digit = u8::from_str_radix(&width, 16).map_err(|e| ValueError::Parse(e));
            match digit {
                Ok(val) => {
                    result_vec.push(val);
                }
                Err(e) => {
                    eprintln!("error: {e}");
                    std::process::exit(1)
                }
            }
        }

        let scaled_values: Vec<u8> = scale(&result_vec, cli_scale);
        print_to_console(&result_vec, &scaled_values, cli_scale);
    }
}

fn handle_rgb(rgb: &str, cli_scale: f32) {
    let mut rgb_iter = rgb.split(",");

    if rgb.contains(":") {
        rgb_iter = rgb.split(":");
    }

    let rgb_split_count: u8 = rgb_iter.clone().count() as u8;
    if rgb_split_count == 3 {
        let red: &str = rgb_iter.next().unwrap();
        let green: &str = rgb_iter.next().unwrap();
        let blue: &str = rgb_iter.next().unwrap();

        let rgb_values: Result<Vec<u8>, ValueError> = convert_to_numeric(&[red, green, blue]);

        match rgb_values {
            Ok(value) => {
                let scaled_values: Vec<u8> = scale(&value, cli_scale);
                print_to_console(&value, &scaled_values, cli_scale);
            }
            Err(e) => {
                println!("{}", e);
            }
        }
    }
}

fn main() {
    let cli: Cli = Cli::parse();

    let input: &str = cli.rgb.as_deref().unwrap_or("0,0,0,0");

    // To limit scale within 0.0 to 1.0.
    // if cli.scale > 1.0 || cli.scale < 0.0 {
    //     eprintln!("Invalid range, Scale value should be within (0.0 -1.0) range.");
    //     std::process::exit(0);
    // }
    //
    if input.contains(",") && input.contains(":") {
        eprintln!("Please use any one of the separate i.e , or :");
        std::process::exit(0);
    }

    if input.contains(",") || input.contains(":") || is_valid_hex(input) {
        if is_valid_hex(input) {
            handle_hex(input, cli.scale);
        } else {
            handle_rgb(input, cli.scale);
        }
    } else {
        eprintln!("Invalid args, use -h for help");
    }
}
