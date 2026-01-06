use std::io::stdout;
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
    match name {"adwaita" => Color::White, "black" => Color::Black, "blue" => Color::Blue, "bluegrey" => Color::Grey, "breeze" => Color::Cyan, "brown" => Color::DarkYellow,
        "carmine" => Color::Red, "cyan" => Color::Cyan, "darkcyan" => Color::DarkCyan, "deeporange" => Color::DarkRed, "green" => Color::Green, "grey" => Color::Grey,
        "indigo" => Color::Magenta, "magenta" => Color::Magenta, "nordic" => Color::Blue, "orange" => Color::Yellow, "palebrown" => Color::Yellow, "paleorange" => Color::Yellow,
        "pink" => Color::Magenta, "red" => Color::Red, "teal" => Color::Cyan, "violet" => Color::Magenta, "white" => Color::White, "yaru" => Color::Cyan, "yellow" => Color::Yellow,
        _ => Color::White,
    }
}

fn print_color_list(colors: &[&str]) {
    for (i, color_name) in colors.iter().enumerate() {
        let colored_name = color_name.with(color_from_name(color_name));
        println!("  [{}] {}", i + 1, colored_name);
    }
}


fn main() {
    clear_screen();
    print_headers();
    print_color_list();
}