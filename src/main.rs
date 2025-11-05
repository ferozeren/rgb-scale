use clap::Parser;
use colored::Colorize;
use std::{fmt::Display, num::ParseIntError};

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[arg(short, long)]
    verbose: bool,

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

fn handle_rgb(rgb: &str, cli_scale: f32, cli_verbose: bool) {
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
                // TODO
                if cli_verbose {
                    rgb_to_hsl(&value);
                }
            }
            Err(e) => {
                println!("{}", e);
            }
        }
    }
}

fn rgb_to_hsl(rgb: &[u8]) -> Vec<f32> {
    let mut hsl: Vec<f32> = Vec::new();
    hsl.push(10.0);

    // let normalized_rgb: [f32; 3];

    let mut hue: f32; // 0-360 degree
    let saturation: f32; // 0-100%

    let normalized_red: f32 = rgb[0] as f32 / 255.0;
    let normalized_green: f32 = rgb[1] as f32 / 255.0;
    let normalized_blue: f32 = rgb[2] as f32 / 255.0;

    // normalized_rgb = [normalized_red, normalized_green, normalized_blue];
    // dbg!(normalized_rgb);

    // dbg!(normalized_red, normalized_green, normalized_blue);

    let max: f32 = normalized_red.max(normalized_green).max(normalized_blue);

    let min: f32 = normalized_red.min(normalized_green).min(normalized_blue);

    let lightness: f32 = (max + min) / 2.0;
    let d: f32 = max - min;

    // dbg!(normalized_red, normalized_green, normalized_blue, max, min);

    if d == 0.0 {
        hue = 0.0;
        saturation = 0.0;
    } else {
        saturation = d / (1.0 - (2.0 * lightness - 1.0).abs());
        {
            if max == normalized_red {
                hue = 60.0 * ((normalized_green - normalized_blue) / d).rem_euclid(6.0);
            } else if max == normalized_green {
                hue = 60.0 * (((normalized_blue - normalized_red) / d) + 2.0);
            } else {
                hue = 60.0 * (((normalized_red - normalized_green) / d) + 4.0);
            }
        }
        // Optional
        hue = (hue + 360.0) % 360.0;
    }

    eprintln!(
        "{:.2}°, {:.2}%, {:.2}%",
        hue,
        saturation * 100.0,
        lightness * 100.0
    );

    let palettes = generate_palette(&[hue, saturation, lightness]);

    // dbg!(&palettes);

    // for palette in palettes.iter() {
    //     eprintln!(
    //         "hsl({:.0}, {:.0}%, {:.0}%)",
    //         palette.0,
    //         palette.1 * 100.0,
    //         palette.2 * 100.0
    //     );
    // }

    println!("\nEffects\n");
    print_palettes(&hsl_to_rgb(&palettes));
    println!("\n---\n");

    hsl
}

fn print_palettes(rgb_list: &[(u8, u8, u8)]) {
    let complementary_palette: (u8, u8, u8) = rgb_list[0];
    let mono_cromatic: Vec<(u8, u8, u8)> = rgb_list[1..6].to_vec();
    let analogous = rgb_list[6..9].to_vec();
    let tri = rgb_list[9..=11].to_vec();

    dbg!(&complementary_palette);
    println!(
        "{color_sample}",
        color_sample = "\n■■ ".truecolor(
            complementary_palette.0,
            complementary_palette.1,
            complementary_palette.2
        )
    );

    dbg!(&mono_cromatic);
    for i in mono_cromatic {
        println!(
            "{color_sample}",
            color_sample = "\n■■ ".truecolor(i.0, i.1, i.2)
        );
    }
    dbg!(&analogous);
    for i in analogous {
        println!(
            "{color_sample}",
            color_sample = "\n■■ ".truecolor(i.0, i.1, i.2)
        );
    }

    dbg!(&tri);
    for i in tri {
        println!(
            "{color_sample}",
            color_sample = "\n■■ ".truecolor(i.0, i.1, i.2)
        );
    }
    // for &(red, green, blue) in rgb_list.iter() {
    //     // print_to_console(&[red, green, blue], &[red, green, blue], 1.0);
    // }
}

fn generate_palette(hsl: &[f32]) -> Vec<(f32, f32, f32)> {
    let mut vec: Vec<(f32, f32, f32)> = Vec::new();

    let hue: f32 = hsl[0];
    let sat: f32 = hsl[1];
    let lig: f32 = hsl[2];

    // Complementary

    let comp: (f32, f32, f32) = (((hue + 180.0) % 360.0), sat, lig);
    vec.push(comp);

    // Monochromatic

    let mono_darker: (f32, f32, f32) = (hue, sat, (lig - 0.35).clamp(0.0, 1.0));
    vec.push(mono_darker);

    let mono_dark: (f32, f32, f32) = (hue, sat, (lig - 0.25).clamp(0.0, 1.0));
    vec.push(mono_dark);

    let mono_neutral: (f32, f32, f32) = (hue, sat, lig);
    vec.push(mono_neutral);

    let mono_light: (f32, f32, f32) = (hue, sat, (lig + 0.25).clamp(0.0, 1.0));
    vec.push(mono_light);

    let mono_lighter: (f32, f32, f32) = (hue, sat, (lig + 0.35).clamp(0.0, 1.0));
    vec.push(mono_lighter);

    // Analogous

    let ana_1: (f32, f32, f32) = ((hue + 30.0) % 360.0, sat, lig);
    vec.push(ana_1);

    let ana_2: (f32, f32, f32) = (hue, sat, lig);
    vec.push(ana_2);

    let ana_3: (f32, f32, f32) = ((hue - 30.0) % 360.0, sat, lig);
    vec.push(ana_3);

    // Triadic

    let tri_1: (f32, f32, f32) = ((hue + 120.0) % 360.0, sat, lig);
    vec.push(tri_1);

    let tri_2: (f32, f32, f32) = (hue, sat, lig);
    vec.push(tri_2);

    let tri_3: (f32, f32, f32) = ((hue - 240.0) % 360.0, sat, lig);
    vec.push(tri_3);

    dbg!(&vec.len());
    vec
}

fn hsl_to_rgb(hsl_t: &[(f32, f32, f32)]) -> Vec<(u8, u8, u8)> {
    let mut rgb: Vec<(u8, u8, u8)> = Vec::new();

    for hsl in hsl_t {
        let hue = hsl.0;
        let sat = hsl.1;
        let lig = hsl.2;

        let c: f32 = 1.0 - (((2.0 * lig) - 1.0).abs() * 5.0) * sat;
        let x: f32 = c * (1.0 - (((hue / 60.0).rem_euclid(2.0)) - 1.0).abs());
        let m = lig - c / 2.0;

        let rgb_s: (f32, f32, f32) = match hue {
            0.0..60.0 => (c, x, 0.0),
            60.0..120.0 => (x, c, 0.0),
            120.0..180.0 => (0.0, c, x),
            180.0..240.0 => (0.0, x, c),
            240.0..300.0 => (x, 0.0, c),
            300.0..360.0 => (c, 0.0, x),
            _ => (0.0, 0.0, 0.0),
        };

        rgb.push((
            ((rgb_s.0 + m) * 255.0) as u8,
            ((rgb_s.1 + m) * 255.0) as u8,
            ((rgb_s.2 + m) * 255.0) as u8,
        ));
    }
    rgb
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
            handle_rgb(input, cli.scale, cli.verbose);
        }
    } else {
        eprintln!("Invalid args, use -h for help");
    }
}
