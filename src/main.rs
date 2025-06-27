use clap::Parser;
use regex::Regex;
use std::{error::Error, fs, path::Path};

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Cli {
    #[arg(long)]
    pub workbench_css_path: String,

    #[arg(long, default_value = "JetBrainsMono Nerd Font Mono")]
    pub font: String,
}

const CLASS_GROUP_NAME: &str = "styles";
const CLASS_PATTERN: &str = r"\.monaco-workbench\.windows(\s+)?\{(?<styles>[^{}]*)\}";
const FONT_GROUP_NAME: &str = "font";
const FONT_PATTERN: &str = r"font-family(\s+)?:(\s+)?(?<font>[^;\n]+)";

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let workbench_css_path = Path::new(&cli.workbench_css_path);
    let css_content = fs::read_to_string(workbench_css_path)?;
    let class_regex = Regex::new(CLASS_PATTERN)?;
    let font_regex = Regex::new(FONT_PATTERN)?;

    if let Some(styles_match) = class_regex
        .captures(&css_content)
        .and_then(|captures| captures.name(CLASS_GROUP_NAME))
    {
        if let Some(font_match) = font_regex
            .captures(styles_match.as_str())
            .and_then(|captures| captures.name(FONT_GROUP_NAME))
        {
            if font_match.as_str() == cli.font {
                println!(
                    "No changes needed: the font is already set to '{}'.",
                    cli.font
                );
            } else {
                let mut new_css_content = String::new();
                let offset_start = styles_match.start() + font_match.start();
                let offset_end = styles_match.start() + font_match.end();

                new_css_content.push_str(&css_content[..offset_start]);
                new_css_content.push_str(&cli.font);
                new_css_content.push_str(&css_content[offset_end..]);

                fs::write(workbench_css_path, new_css_content)?;
                println!("Font updated to '{}'.", cli.font);
            }
        } else {
            eprintln!("font-family not found inside .monaco-workbench.windows block.");
        }
    } else {
        eprintln!("Class .monaco-workbench.windows {{...}} not found.");
    }

    Ok(())
}
