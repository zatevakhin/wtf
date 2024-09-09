use std::io::{stdout, Write};
use std::time::Duration;
use std::{env, thread};
use crossterm::{
    cursor, execute,
    style::{Color, SetForegroundColor},
    terminal::{Clear, ClearType},
    Result,
};
use rand::Rng;

enum ColorScheme {
    Random,
    Rainbow,
    TransPride,
    Ukraine,
}

fn clear_screen() -> Result<()> {
    execute!(stdout(), Clear(ClearType::All), cursor::MoveTo(0, 0))
}

fn print_at(x: u16, y: u16, text: &str, color: Color) -> Result<()> {
    execute!(
        stdout(),
        cursor::MoveTo(x, y),
        SetForegroundColor(color),
        crossterm::style::Print(text)
    )
}

fn random_color() -> Color {
    let mut rng = rand::thread_rng();
    Color::AnsiValue(rng.gen_range(16..232))
}

fn rainbow_color(y: u16, height: u16) -> Color {
    let colors = [
        Color::Red,
        Color::Yellow,
        Color::Green,
        Color::Cyan,
        Color::Blue,
        Color::Magenta,
        Color::Red,
    ];
    let section_height = height / 7;
    let color_index = std::cmp::min(y / section_height, 6) as usize;
    colors[color_index]
}

fn trans_pride_color(y: u16, height: u16) -> Color {
    let colors = [
        Color::Rgb { r: 91, g: 206, b: 250 }, // Light Blue
        Color::Rgb { r: 245, g: 169, b: 184 }, // Light Pink
        Color::White,
        Color::Rgb { r: 245, g: 169, b: 184 }, // Light Pink
        Color::Rgb { r: 91, g: 206, b: 250 }, // Light Blue
    ];
    let section_height = height / 5;
    let color_index = std::cmp::min(y / section_height, 4) as usize;
    colors[color_index]
}

fn ukraine_color(y: u16, height: u16) -> Color {
    let colors = [
        Color::Rgb { r: 0, g: 87, b: 183 },  // Blue
        Color::Rgb { r: 255, g: 215, b: 0 }, // Yellow
    ];
    let section_height = height / 2;
    let color_index = if y < section_height { 0 } else { 1 };
    colors[color_index]
}

fn get_color(y: u16, height: u16, scheme: &ColorScheme) -> Color {
    match scheme {
        ColorScheme::Random => random_color(),
        ColorScheme::Rainbow => rainbow_color(y, height),
        ColorScheme::TransPride => trans_pride_color(y, height),
        ColorScheme::Ukraine => ukraine_color(y, height),
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <random|rainbow|trans_pride>", args[0]);
        std::process::exit(1);
    }

    let scheme = match args[1].as_str() {
        "random" => ColorScheme::Random,
        "rainbow" => ColorScheme::Rainbow,
        "trans_pride" => ColorScheme::TransPride,
        "ukraine" => ColorScheme::Ukraine,
        _ => {
            eprintln!("Invalid color scheme. Choose from: random, rainbow, trans_pride, ukraine");
            std::process::exit(1);
        }
    };

    clear_screen()?;

    let (width, height) = crossterm::terminal::size()?;
    let mut rng = rand::thread_rng();

    loop {
        let x = rng.gen_range(0..width - 3);
        let y = rng.gen_range(0..height);
        let color = get_color(y, height, &scheme);
        print_at(x, y, "wat", color)?;
        stdout().flush()?;
        thread::sleep(Duration::from_millis(100));
    }
}
