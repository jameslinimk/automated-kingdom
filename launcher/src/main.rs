use std::fs::canonicalize;
use std::io;
use std::process::Command;

use colored::Colorize;
use zip::result::ZipResult;
use zip::ZipArchive;

fn main() -> ZipResult<()> {
    println!("{}", "Automated Kingdom".red().bold());
    println!(
        "{} {}",
        "By Linimik".blue(),
        "(Discord: Linimik#8661)".black()
    );
    println!();

    // Extract files
    println!("{}", "Extracting files...".green());
    let zip_file = include_bytes!("../out.zip");
    let mut archive = ZipArchive::new(io::Cursor::new(zip_file))?;
    archive.extract("automated_kingdom")?;

    // Run the game
    let path = canonicalize("automated_kingdom/automated-kingdom.exe")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    println!("{}", "Opening the game...".green());
    Command::new(path)
        .spawn()
        .expect("Command failed to start!");

    Ok(())
}
