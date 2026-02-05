use owo_colors::OwoColorize;
use std::process::Command;

use crate::characters::Character;

pub fn render_with_chafa(path: &str, size: Option<(u16, u16)>) -> std::io::Result<()> {
    let mut cmd = Command::new("chafa");

    // Quality-ish defaults that usually look clean:
    cmd.arg("--symbols=block").arg("--colors=full"); // cmd.arg("--dither=none");

    if let Some((w, h)) = size {
        cmd.arg(format!("--size={}x{}", w, h));
    }

    cmd.arg(path);

    let status = cmd.status()?;
    if !status.success() {
        eprintln!("chafa failed (is it installed? is the image path correct?)");
    }
    Ok(())
}

pub fn print_character(ch: &Character) {
    // Render image first
    if let Err(e) = render_with_chafa(ch.image_path, Some((40, 40))) {
        eprintln!("Could not run chafa: {e}");
    }

    // Then text
    println!();
    println!("{} {}", "Name:".bold(), ch.name.bold());
    println!("{} {}", "Quote:".bold(), format!("“{}”", ch.quote).italic());
}
