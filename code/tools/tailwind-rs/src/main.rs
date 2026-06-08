mod types;

use types::{Color, Shade};

use std::collections::HashMap;
use std::fs;
use std::io::Write;

use palette::{FromColor, OklabHue, Oklch, Srgb};
use serde_json::Value;

const SOURCE: &str = include_str!("../raw/colors.json");

/// Split `oklch(64.8% 0.2 131.684)` into ("64.8", "0.2", "131.684").
fn parse_oklch_str(oklch: &str) -> Option<(&str, &str, &str)> {
    let val = oklch.strip_prefix("oklch(")?;
    let val = val.strip_suffix(')')?;
    let (a, b) = val.split_once(' ')?;
    let (b, c) = b.split_once(' ')?;
    Some((a.strip_suffix('%')?, b, c))
}

fn parse() -> HashMap<Color, HashMap<Shade, Oklch<f64>>> {
    let data: HashMap<String, Value> = serde_json::from_str(SOURCE).unwrap();
    let mut colors = data.keys().map(|v| v.as_str()).collect::<Vec<_>>();
    colors.retain(|k| data.get(*k).unwrap().is_object());

    let mut output = HashMap::new();

    for color in colors {
        let map = data[color].as_object().unwrap();
        let mut shades: Vec<_> = map.keys().map(|v| v.as_str()).collect();
        shades.sort_by_key(|v| v.parse::<u32>().unwrap());

        let mut color_map = HashMap::new();

        for shade in shades {
            let oklch_value = map.get(shade).unwrap().as_str().unwrap();
            let (a, b, c) = parse_oklch_str(oklch_value).unwrap();
            let a = a.parse::<f64>().unwrap() / 100.;
            let b = b.parse::<f64>().unwrap();
            let c = c.parse::<f64>().unwrap();

            let oklch = Oklch::new(a, b, c);
            let rgb = Srgb::from_color(oklch);

            let oklch2 = Oklch::new_const(a, b, OklabHue::from_degrees(c));
            let rgb2 = Srgb::from_color(oklch2);

            let oklch3 = Oklch::new_const(
                oklch.l,
                oklch.chroma,
                OklabHue::new(oklch.hue.into_raw_degrees()),
            );
            let rgb3 = Srgb::from_color(oklch3);

            assert_eq!(rgb, rgb2);
            assert_eq!(rgb, rgb3);
            color_map.insert(Shade::new(shade), oklch2);
        }
        output.insert(Color::new(color), color_map);
    }
    output
}

fn cap1(x: &str) -> String {
    let (a, b) = x.split_at(1);
    format!("{}{}", a.to_uppercase(), b)
}

fn print_rust<W: Write>(f: &mut W, data: &HashMap<Color, HashMap<Shade, Oklch<f64>>>) {
    writeln!(f, "{}", "use super::*;").unwrap();

    let mut n = 0;

    for (color, value) in data {
        for (shade, oklch) in value {
            n += 1;
            let color_struct = format!("{}_{shade}", color.as_str().to_uppercase());
            write!(f, "pub const {color_struct}: Color = Color {{").unwrap();
            write!(f, "name: \"{}{shade}\",", cap1(color.as_str())).unwrap();
            write!(
                f,
                "oklch: Oklch::new_const({a:?}, {b:?}, OklabHue::new({c:?}))",
                a = oklch.l,
                b = oklch.chroma,
                c = oklch.hue.into_raw_degrees(),
            )
            .unwrap();
            writeln!(f, "}};").unwrap();
        }
    }

    write!(f, "pub const ALL_COLORS: [Color; {n}] = [").unwrap();
    for (color, value) in data {
        for shade in value.keys() {
            let color_struct = format!("{}_{shade}", color.as_str().to_uppercase());
            write!(f, "{color_struct},").unwrap();
        }
    }
    writeln!(f, "];").unwrap();
}

fn print_latex<W: Write>(f: &mut W, data: &HashMap<Color, HashMap<Shade, Oklch<f64>>>) {
    for (color, value) in data {
        for (shade, oklch) in value {
            let color_struct = format!("{}{shade}", cap1(color.as_str()));
            let rgb = Srgb::from_color(oklch.clone());
            writeln!(
                f,
                "\\definecolor{{{}}}{{HTML}}{{{R:0>2X}{G:0>2X}{B:0>2X}}}",
                color_struct,
                R = (rgb.red * 255.) as i64,
                G = (rgb.green * 255.) as i64,
                B = (rgb.blue * 255.) as i64,
            )
            .unwrap();
        }
    }
}

fn main() {
    let data = parse();

    let mut f = fs::File::create("tailwind-output.rs").unwrap();
    print_rust(&mut f, &data);
    let mut f = fs::File::create("tailwind-output.tex").unwrap();
    print_latex(&mut f, &data);
}
