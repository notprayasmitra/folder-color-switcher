use std::io::stdout;
use std::process::Command;
use crossterm::{
    cursor::{MoveTo, Hide, Show},
    event::{self, Event, KeyCode},
    execute,
    style::{Color, Stylize, Print},
    terminal::{Clear, ClearType, enable_raw_mode, disable_raw_mode},
};

fn run_app() {
    let colors = folder_colors();
    
    // Show loading message
    clear_screen();
    print_loading();
    
    let current_color_name = get_current_color();

    let current_index = colors
        .iter()
        .position(|&c| c == current_color_name)
        .unwrap_or(0);

    let mut selected_index = current_index;
    let mut error_message: Option<String> = None;
    let mut filter = String::new();
    let mut search_mode = false;

    loop {
        clear_screen();
        print_headers();
        print_instructions();
        
        let filtered_colors: Vec<(usize, &str)> = if filter.is_empty() {
            colors.iter().enumerate().map(|(i, &c)| (i, c)).collect()
        } else {
            colors.iter().enumerate()
                .filter(|(_, c)| c.contains(&filter.to_lowercase()))
                .map(|(i, &c)| (i, c))
                .collect()
        };

        print_color_list(&filtered_colors, selected_index, current_index);
        
        // Show search bar if in search mode
        if search_mode {
            print_search_bar(&filter);
        }

        // Display error message if any
        if let Some(ref msg) = error_message {
            print_error_box(msg, colors.len());
        }
        
        print_footer();

        match handle_input(&mut selected_index, &filtered_colors, &mut filter, &mut search_mode, colors.len()) {
            Action::Apply => {
                let actual_index = if filter.is_empty() {
                    selected_index
                } else {
                    filtered_colors.iter()
                        .find(|(i, _)| *i == selected_index)
                        .map(|(i, _)| *i)
                        .unwrap_or(selected_index)
                };
                
                // Show confirmation
                if !show_confirmation(colors[actual_index]) {
                    continue;
                }
                
                // Restore terminal before running command
                restore_terminal();
                clear_screen();
                println!("\n╔═══════════════════════════════════════════╗");
                println!("║         Applying Color Change...          ║");
                println!("╚═══════════════════════════════════════════╝\n");
                print!("Applying {} ", colors[actual_index]);
                print_color_block(color_from_name(colors[actual_index]));
                println!(" ...\n");
                
                match set_color(colors[actual_index]) {
                    Ok(_) => {
                        println!("✓ Color applied successfully!");
                        std::thread::sleep(std::time::Duration::from_millis(1000));
                        break;
                    }
                    Err(e) => {
                        println!("✗ Error: {}", e);
                        println!("\nPress Enter to continue...");
                        let _ = std::io::stdin().read_line(&mut String::new());
                        
                        // Re-setup terminal and continue
                        setup_terminal();
                        error_message = Some(e);
                    }
                }
            }
            Action::Exit => break,
            Action::ToggleSearch => {
                search_mode = !search_mode;
                if !search_mode {
                    filter.clear();
                }
                error_message = None;
            }
            Action::None => {
                error_message = None;
            }
        }
    }
}

fn setup_terminal() {
    enable_raw_mode().unwrap();
    execute!(stdout(), Hide).unwrap();
}

fn restore_terminal() {
    disable_raw_mode().unwrap();
    execute!(stdout(), Show).unwrap();
}

fn clear_screen() {
    execute!(
        stdout(),
        Clear(ClearType::All),
        MoveTo(0, 0)
    )
    .unwrap();
}

fn print_loading() {
    execute!(
        stdout(),
        MoveTo(0, 5),
        Print("╔═══════════════════════════════════════════╗"),
        MoveTo(0, 6),
        Print("║       Papirus Folder Color Switcher       ║"),
        MoveTo(0, 7),
        Print("╚═══════════════════════════════════════════╝"),
        MoveTo(0, 9),
        Print("  Detecting current color...".with(Color::Cyan)),
    )
    .unwrap();
}

fn print_headers() {
    execute!(
        stdout(),
        MoveTo(0, 0),
        Print("╔═══════════════════════════════════════════╗"),
        MoveTo(0, 1),
        Print("║       Papirus Folder Color Switcher       ║"),
        MoveTo(0, 2),
        Print("╚═══════════════════════════════════════════╝"),
    )
    .unwrap();
}

fn print_instructions() {
    execute!(
        stdout(),
        MoveTo(0, 3),
        Print("  Use ↑↓ to navigate, Enter to apply".dark_grey()),
    )
    .unwrap();
}

fn print_footer() {
    execute!(
        stdout(),
        MoveTo(0, 32),
        Print("─".repeat(47).dark_grey()),
        MoveTo(0, 33),
        Print("  /: Search  |  Enter: Apply  |  Q/Esc: Quit".dark_grey()),
    )
    .unwrap();
}

fn print_search_bar(filter: &str) {
    execute!(
        stdout(),
        MoveTo(0, 30),
        Print("─".repeat(47).with(Color::Cyan)),
        MoveTo(0, 31),
        Print(format!("  Search: {}_", filter).with(Color::Cyan)),
    )
    .unwrap();
}

fn print_error_box(msg: &str, _colors_len: usize) {
    let lines: Vec<&str> = msg.lines().collect();
    let max_width = lines.iter().map(|l| l.len()).max().unwrap_or(0).min(43);
    let box_width = max_width + 4;
    
    let top = format!("╔{}╗", "═".repeat(box_width - 2));
    let bottom = format!("╚{}╝", "═".repeat(box_width - 2));
    
    execute!(
        stdout(),
        MoveTo(2, 10),
        Print(top.with(Color::Red)),
    )
    .unwrap();
    
    for (i, line) in lines.iter().enumerate() {
        let padded = format!("║ {:width$} ║", line, width = box_width - 4);
        execute!(
            stdout(),
            MoveTo(2, 11 + i as u16),
            Print(padded.with(Color::Red)),
        )
        .unwrap();
    }
    
    execute!(
        stdout(),
        MoveTo(2, 11 + lines.len() as u16),
        Print(bottom.with(Color::Red)),
    )
    .unwrap();
}

fn show_confirmation(color: &str) -> bool {
    clear_screen();
    
    let color_rgb = color_from_name(color);
    
    execute!(
        stdout(),
        MoveTo(10, 8),
        Print("╔═══════════════════════════════╗"),
        MoveTo(10, 9),
        Print("║      Confirm Color Change     ║"),
        MoveTo(10, 10),
        Print("╠═══════════════════════════════╣"),
        MoveTo(10, 11),
        Print("║                               ║"),
        MoveTo(10, 12),
        Print("║  Apply color: "),
    )
    .unwrap();
    
    print_color_block(color_rgb);
    
    let color_text = format!(" {}", color).with(color_rgb);
    let padding = " ".repeat(31 - 15 - 3 - 1 - color.len());
    
    execute!(
        stdout(),
        Print(color_text),
        Print(padding),
        Print("║"),
        MoveTo(10, 13),
        Print("║                               ║"),
        MoveTo(10, 14),
        Print("╠═══════════════════════════════╣"),
        MoveTo(10, 15),
        Print("║  Enter: Confirm  Esc: Cancel  ║"),
        MoveTo(10, 16),
        Print("╚═══════════════════════════════╝"),
    )
    .unwrap();
    
    loop {
        if let Event::Key(key) = event::read().unwrap() {
            match key.code {
                KeyCode::Enter => return true,
                KeyCode::Esc | KeyCode::Char('q') => return false,
                _ => {}
            }
        }
    }
}

fn folder_colors() -> Vec<&'static str> {
    vec![
        "adwaita", "black", "blue", "bluegrey", "breeze", 
        "brown", "carmine", "cyan", "darkcyan", "deeporange", 
        "green", "grey", "indigo", "magenta", "nordic", 
        "orange", "palebrown", "paleorange", "pink", "red", 
        "teal", "violet", "white", "yaru", "yellow"
    ]
}

fn color_from_name(name: &str) -> Color {
    match name {
        "adwaita"    => Color::Rgb { r: 0x93, g: 0xC0, b: 0xEA },
        "black"      => Color::Rgb { r: 0x4F, g: 0x4F, b: 0x4F },
        "blue"       => Color::Rgb { r: 0x52, g: 0x94, b: 0xE2 },
        "bluegrey"   => Color::Rgb { r: 0x60, g: 0x7D, b: 0x8B },
        "breeze"     => Color::Rgb { r: 0x57, g: 0xB8, b: 0xEC },
        "brown"      => Color::Rgb { r: 0xAE, g: 0x8E, b: 0x6C },
        "carmine"    => Color::Rgb { r: 0xA3, g: 0x00, b: 0x02 },
        "cyan"       => Color::Rgb { r: 0x00, g: 0xBC, b: 0xD4 },
        "darkcyan"   => Color::Rgb { r: 0x45, g: 0xAB, b: 0xB7 },
        "deeporange" => Color::Rgb { r: 0xEB, g: 0x66, b: 0x37 },
        "green"      => Color::Rgb { r: 0x87, g: 0xB1, b: 0x58 },
        "grey"       => Color::Rgb { r: 0x8E, g: 0x8E, b: 0x8E },
        "indigo"     => Color::Rgb { r: 0x5C, g: 0x6B, b: 0xC0 },
        "magenta"    => Color::Rgb { r: 0xCA, g: 0x71, b: 0xDF },
        "nordic"     => Color::Rgb { r: 0x81, g: 0xA1, b: 0xC1 },
        "orange"     => Color::Rgb { r: 0xEE, g: 0x92, b: 0x3A },
        "palebrown"  => Color::Rgb { r: 0xD1, g: 0xBF, b: 0xAE },
        "paleorange" => Color::Rgb { r: 0xEE, g: 0xCA, b: 0x8F },
        "pink"       => Color::Rgb { r: 0xF0, g: 0x62, b: 0x92 },
        "red"        => Color::Rgb { r: 0xE2, g: 0x52, b: 0x52 },
        "teal"       => Color::Rgb { r: 0x16, g: 0xA0, b: 0x85 },
        "violet"     => Color::Rgb { r: 0x7E, g: 0x57, b: 0xC2 },
        "white"      => Color::Rgb { r: 0xE4, g: 0xE4, b: 0xE4 },
        "yaru"       => Color::Rgb { r: 0x97, g: 0x35, b: 0x52 },
        "yellow"     => Color::Rgb { r: 0xF9, g: 0xBD, b: 0x30 },
        _            => Color::White,
    }
}

fn print_color_block(color: Color) {
    execute!(stdout(), Print("███".with(color))).unwrap();
}

fn print_color_list(
    colors: &[(usize, &str)],
    selected_index: usize,
    current_index: usize,
) {
    let mut y = 5;

    for &(original_index, color_name) in colors.iter() {
        execute!(stdout(), MoveTo(0, y)).unwrap();

        let color_rgb = color_from_name(color_name);
        let arrow = if original_index == selected_index { "►" } else { " " };
        let status = if original_index == current_index { 
            "[ACTIVE]".with(Color::Green)
        } else { 
            "        ".with(Color::White)
        };

        execute!(
            stdout(),
            Print(format!(" {} ", arrow).with(Color::Yellow)),
        )
        .unwrap();
        
        print_color_block(color_rgb);
        
        execute!(
            stdout(),
            Print(format!("  {:<12} ", color_name).with(color_rgb)),
            Print(status),
        )
        .unwrap();
        
        println!();
        y += 1;
    }
}

fn get_current_color() -> String {
    let output = Command::new("papirus-folders")
        .arg("-l")
        .arg("--theme")
        .arg("Papirus-Dark")
        .output();
    
    if let Ok(output) = output {
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            for line in stdout.lines() {
                let trimmed = line.trim();
                if trimmed.starts_with('>') {
                    return trimmed[1..].trim().to_string();
                }
            }
        }
    }

    String::from("adwaita")
}

enum Action {
    None,
    Apply,
    Exit,
    ToggleSearch,
}

fn set_color(color: &str) -> Result<(), String> {
    let output = Command::new("papirus-folders")
        .arg("-t")
        .arg("Papirus-Dark")
        .arg("--color")
        .arg(color)
        .output()
        .map_err(|e| format!("Failed to execute papirus-folders: {}. Is it installed?", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        return Err(format!("Command failed: {}{}", stderr, stdout));
    }

    Ok(())
}

fn handle_input(
    selected: &mut usize, 
    filtered_colors: &[(usize, &str)],
    filter: &mut String,
    search_mode: &mut bool,
    _max: usize
) -> Action {
    if let Event::Key(key) = event::read().unwrap() {
        if *search_mode {
            match key.code {
                KeyCode::Char(c) => {
                    filter.push(c);
                    Action::None
                }
                KeyCode::Backspace => {
                    filter.pop();
                    Action::None
                }
                KeyCode::Esc => {
                    *search_mode = false;
                    filter.clear();
                    Action::None
                }
                KeyCode::Enter if !filtered_colors.is_empty() => {
                    *search_mode = false;
                    *selected = filtered_colors[0].0;
                    Action::None
                }
                _ => Action::None,
            }
        } else {
            match key.code {
                KeyCode::Up => {
                    if let Some(current_pos) = filtered_colors.iter().position(|(i, _)| *i == *selected) {
                        if current_pos > 0 {
                            *selected = filtered_colors[current_pos - 1].0;
                        }
                    }
                    Action::None
                }
                KeyCode::Down => {
                    if let Some(current_pos) = filtered_colors.iter().position(|(i, _)| *i == *selected) {
                        if current_pos < filtered_colors.len() - 1 {
                            *selected = filtered_colors[current_pos + 1].0;
                        }
                    }
                    Action::None
                }
                KeyCode::Enter => Action::Apply,
                KeyCode::Esc | KeyCode::Char('q') => Action::Exit,
                KeyCode::Char('/') => Action::ToggleSearch,
                _ => Action::None,
            }
        }
    } else {
        Action::None
    }
}

fn main() {
    setup_terminal();

    let result = std::panic::catch_unwind(|| {
        run_app();
    });

    restore_terminal();
    
    if result.is_err() {
        eprintln!("Application panicked!");
    }
}