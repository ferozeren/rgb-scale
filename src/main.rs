use clap::Parser;
use colored::Colorize;
use std::{fmt::Display, num::ParseIntError};

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[arg(short, long, default_value_t = 1.0)]
    scale: f32,
    /// RGB Value (e.g., "255,128,64")
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

fn convert_to_numeric(color_value: &[&str]) -> Result<Vec<u8>, ValueError> {
    let mut numeric_vec: Vec<u8> = Vec::new();
    for idx in color_value {
        let digit: u8 = idx.parse().map_err(|e| ValueError::Parse(e))?;
        numeric_vec.push(digit);
    }
    Ok(numeric_vec)
}

fn main() {
    let cli: Cli = Cli::parse();

    let rgb: &str = cli.rgb.as_deref().unwrap_or("0,0,0,0");

    // if cli.scale > 1.0 || cli.scale < 0.0 {
    //     eprintln!("Invalid range, Scale value should be within (0.0 -1.0) range.");
    //     std::process::exit(0);
    // }

    if rgb.contains(",") && rgb.contains(":") {
        eprintln!("Please use any one of the separate i.e , or :");
        std::process::exit(0);
    }

    if rgb.contains(",") || rgb.contains(":") {
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
                    let mut scaled_values: Vec<u8> = Vec::new();

                    for val in value.clone() {
                        let v: f32 = val as f32 * cli.scale;
                        scaled_values.push(v as u8)
                    }
                    print!(
                        "{color_sample}",
                        color_sample = "\n■■ ".truecolor(value[0], value[1], value[2])
                    );
                    println!("Original Value:\n{},{},{}\n", value[0], value[1], value[2]);
                    print!(
                        "{color_sample}",
                        color_sample =
                            "■■ ".truecolor(scaled_values[0], scaled_values[1], scaled_values[2])
                    );
                    println!(
                        "Scaled Value ({}):\n{},{},{}",
                        cli.scale, scaled_values[0], scaled_values[1], scaled_values[2]
                    );
                }
                Err(e) => {
                    println!("{}", e);
                }
            }
        }
    } else {
        eprintln!("Invalid args, use -h for help");
    }
}
