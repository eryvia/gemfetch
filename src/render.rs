use owo_colors::OwoColorize;
use std::process::Command;
use crate::characters::Character;

pub fn render_with_chafa(path: &str, size: Option<(u16, u16)>) -> std::io::Result<()> {
    let mut cmd = Command::new("chafa");
    cmd.arg("--symbols=block").arg("--colors=full");

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
    // Note: changed ch.image_path to &ch.image_path because render_with_chafa takes &str
    if let Err(e) = render_with_chafa(&ch.image_path, Some((40, 20))) { 
        eprintln!("Could not run chafa: {e}");
    }

    println!();
    println!("{} {}", "Name:".bold().cyan(), ch.character.bold());
    println!("{} {}", "Source:".bold().magenta(), ch.source);
    println!("{} {}", "Quote:".bold().yellow(), format!("“{}”", ch.quote).italic());
    println!();
}
