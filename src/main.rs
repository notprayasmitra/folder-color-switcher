use std::io::stdout;
use std::process::Command;
use crossterm::{
    execute,
    terminal::{Clear, ClearType},
    cursor::MoveTo,
};
use crossterm::style::{Color, Stylize};

fn clear_screen() {
    execute!(
        stdout(),
        Clear(ClearType::All),
        MoveTo(0, 0)
    )
    .unwrap();
}

fn print_headers() {
    println!("╔═══════════════════════════════════════════╗");
    println!("║       Papirus Folder Color Switcher       ║");
    println!("╚═══════════════════════════════════════════╝\n");
}

fn folder_colors() -> Vec<&'static str> {
    vec!["adwaita", "black", "blue", "bluegrey", "breeze", "brown", "carmine", "cyan", "darkcyan", "deeporange", "green", "grey", "indigo", "magenta", "nordic", "orange", 
    "palebrown", "paleorange", "pink", "red", "teal", "violet", "white", "yaru", "yellow"]
}

fn color_from_name(name: &str) -> Color {
    match name {
        "adwaita"    => Color::Rgb { r: 0x93, g: 0xC0, b: 0xEA }, // #93c0ea
        "black"      => Color::Rgb { r: 0x4F, g: 0x4F, b: 0x4F }, // #4f4f4f
        "blue"       => Color::Rgb { r: 0x52, g: 0x94, b: 0xE2 }, // #5294e2
        "bluegrey"   => Color::Rgb { r: 0x60, g: 0x7D, b: 0x8B }, // #607d8b
        "breeze"     => Color::Rgb { r: 0x57, g: 0xB8, b: 0xEC }, // #57b8ec
        "brown"      => Color::Rgb { r: 0xAE, g: 0x8E, b: 0x6C }, // #ae8e6c
        "carmine"    => Color::Rgb { r: 0xA3, g: 0x00, b: 0x02 }, // #a30002
        "cyan"       => Color::Rgb { r: 0x00, g: 0xBC, b: 0xD4 }, // #00bcd4
        "darkcyan"   => Color::Rgb { r: 0x45, g: 0xAB, b: 0xB7 }, // #45abb7
        "deeporange" => Color::Rgb { r: 0xEB, g: 0x66, b: 0x37 }, // #eb6637
        "green"      => Color::Rgb { r: 0x87, g: 0xB1, b: 0x58 }, // #87b158
        "grey"       => Color::Rgb { r: 0x8E, g: 0x8E, b: 0x8E }, // #8e8e8e
        "indigo"     => Color::Rgb { r: 0x5C, g: 0x6B, b: 0xC0 }, // #5c6bc0
        "magenta"    => Color::Rgb { r: 0xCA, g: 0x71, b: 0xDF }, // #ca71df
        "nordic"     => Color::Rgb { r: 0x81, g: 0xA1, b: 0xC1 }, // #81a1c1
        "orange"     => Color::Rgb { r: 0xEE, g: 0x92, b: 0x3A }, // #ee923a
        "palebrown"  => Color::Rgb { r: 0xD1, g: 0xBF, b: 0xAE }, // #d1bfae
        "paleorange" => Color::Rgb { r: 0xEE, g: 0xCA, b: 0x8F }, // #eeca8f
        "pink"       => Color::Rgb { r: 0xF0, g: 0x62, b: 0x92 }, // #f06292
        "red"        => Color::Rgb { r: 0xE2, g: 0x52, b: 0x52 }, // #e25252
        "teal"       => Color::Rgb { r: 0x16, g: 0xA0, b: 0x85 }, // #16a085
        "violet"     => Color::Rgb { r: 0x7E, g: 0x57, b: 0xC2 }, // #7e57c2
        "white"      => Color::Rgb { r: 0xE4, g: 0xE4, b: 0xE4 }, // #e4e4e4
        "yaru"       => Color::Rgb { r: 0x97, g: 0x35, b: 0x52 }, // #973552
        "yellow"     => Color::Rgb { r: 0xF9, g: 0xBD, b: 0x30 }, // #f9bd30
        _            => Color::White,
    }
}

fn print_color_list(colors: &[&str], current_index: usize) {
    for (i, color_name) in colors.iter().enumerate() {
        let colored_name = color_name.with(color_from_name(color_name)); // Hollow dot is unselected, while filled fot is crrent folder color
        let dot = if i == current_index { "●" } else { "○" };
        println!("  {} {}", dot, colored_name);
    }
}

fn get_current_color() -> String {
    let output = Command::new("scripts/papirus-fetch.sh")
        .arg("Papirus-Dark")
        .arg("list")
        .output()
        .expect("Failed to run papirus-fetch.sh");

    let stdout = String::from_utf8_lossy(&output.stdout);

    for line in stdout.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('>') {
            return trimmed[1..].trim().to_string();
        }
    }

    String::new() // fallback string
}

fn main() {
    clear_screen();
    print_headers();

    let colors = folder_colors();
    // let current_color_index = 3; //hardcoded string

    let current_color_name = get_current_color();

    let current_color_index = colors
        .iter()
        .position(|&c| c == current_color_name)
        .unwrap_or(0); // fallback if not found

    print_color_list(&colors, current_color_index);
}