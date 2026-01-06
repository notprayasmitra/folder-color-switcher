use std::process::Command;
use std::io::{self, Write};

const COLORS: &[&str] = &[
    "black", "blue", "bluegrey", "breeze", "brown", "carmine", "cyan",
    "darkcyan", "deeporange", "green", "grey", "indigo", "magenta",
    "nordic", "orange", "palebrown", "paleorange", "pink", "red",
    "teal", "violet", "white", "yellow", "yaru"
];

fn main() {
    loop {
        clear_screen();
        print_header();
        print_colors();
        
        match get_user_choice() {
            Ok(choice) => {
                if choice == 0 {
                    println!("\nExiting...");
                    break;
                }
                
                if choice > 0 && choice <= COLORS.len() {
                    let selected_color = COLORS[choice - 1];
                    apply_color(selected_color);
                } else {
                    println!("\nInvalid choice! Press Enter to continue...");
                    wait_for_enter();
                }
            }
            Err(_) => {
                println!("\nInvalid input! Press Enter to continue...");
                wait_for_enter();
            }
        }
    }
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}

fn print_header() {
    println!("╔═══════════════════════════════════════════╗");
    println!("║   Papirus Folder Color Changer (TUI)     ║");
    println!("╚═══════════════════════════════════════════╝\n");
}

fn print_colors() {
    println!("Available colors:\n");
    
    for (i, color) in COLORS.iter().enumerate() {
        print!("  [{:2}] {:12}", i + 1, color);
        if (i + 1) % 3 == 0 {
            println!();
        }
    }
    println!("\n\n  [0] Exit\n");
    println!("─────────────────────────────────────────────");
}

fn get_user_choice() -> Result<usize, std::num::ParseIntError> {
    print!("Enter your choice: ");
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    input.trim().parse()
}

fn apply_color(color: &str) {
    println!("\nApplying color '{}'...", color);
    
    let output = Command::new("papirus-folders")
        .arg("-C")
        .arg(color)
        .output();
    
    match output {
        Ok(result) => {
            if result.status.success() {
                println!("✓ Color applied successfully!");
            } else {
                println!("✗ Error applying color:");
                println!("{}", String::from_utf8_lossy(&result.stderr));
            }
        }
        Err(e) => {
            println!("✗ Failed to execute papirus-folders: {}", e);
            println!("Make sure papirus-folders is installed:");
            println!("  sudo pacman -S papirus-folders");
        }
    }
    
    println!("\nPress Enter to continue...");
    wait_for_enter();
}

fn wait_for_enter() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}