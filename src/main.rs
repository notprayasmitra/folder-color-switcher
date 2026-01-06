use std::io::{self, Write};

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}

fn print_headers() {
    println!("╔═══════════════════════════════════════════╗");
    println!("║       Papirus Folder Color Switcher       ║");
    println!("╚═══════════════════════════════════════════╝\n");
}

fn main() {
    clear_screen();
    print_headers();
}