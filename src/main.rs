// src/main.rs
use clap::{Parser, Subcommand};
use conceptior::{generate_menu, generate_window, load_asset_package, models::Menu};

/// Conceptior: A tool for generating UI concepts
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a new menu
    GenerateMenu {
        title: String,
        items: Vec<String>,
    },
    /// Generate a new window
    GenerateWindow {
        title: String,
        width: u32,
        height: u32,
        menus: Vec<String>,
    },
    /// Load an asset package
    LoadPackage {
        path: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::GenerateMenu { title, items } => {
            println!("Generating menu with title: {}", title);
            let items: Vec<(&str, &str)> = items.iter().map(|s| {
                let parts: Vec<&str> = s.split(':').collect();
                (parts[0], parts[1])
            }).collect();
            let menu = generate_menu(title, items);
            println!("Generated Menu: {:#?}", menu);
        },
        Commands::GenerateWindow { title, width, height, menus } => {
            println!("Generating window with title: {}, width: {}, height: {}", title, width, height);
            let menus: Vec<Menu> = menus.iter().map(|s| {
                // For simplicity, assume the menus are predefined and identified by name
                println!("Adding menu: {}", s);
                generate_menu(s, vec![("Item1", "Action1"), ("Item2", "Action2")])
            }).collect();
            let window = generate_window(title, *width, *height, menus);
            println!("Generated Window: {:#?}", window);
        },
        Commands::LoadPackage { path } => {
            println!("Loading package from path: {}", path);
            match load_asset_package(path) {
                Ok(package) => println!("Loaded Package: {:#?}", package),
                Err(e) => eprintln!("Error loading package: {}", e),
            }
        },
    }
}
