use anyhow::{Context, Result};
use clap::Parser;
use image::Luma;
use qrcode::QrCode;
use std::path::PathBuf;

/// A CLI tool to convert text data into QR codes
#[derive(Parser, Debug)]
#[command(name = "createqrcode")]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The data/text to encode in the QR code
    #[arg(short, long)]
    data: String,

    /// Output file path (e.g., output.png)
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Scale factor for the QR code (higher = larger image)
    #[arg(short, long, default_value = "10")]
    scale: u32,

    /// Border size (in modules/blocks)
    #[arg(short, long, default_value = "4")]
    border: u32,

    /// Print QR code to terminal/console
    #[arg(short, long)]
    print: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Validate that at least one output method is specified
    if !args.print && args.output.is_none() {
        anyhow::bail!("Either --print flag or --output path must be specified");
    }

    // Validate scale and border
    if args.scale == 0 {
        anyhow::bail!("Scale must be greater than 0");
    }

    // Generate QR code
    println!("Generating QR code for: {}", args.data);
    
    let code = QrCode::new(&args.data)
        .context("Failed to generate QR code. The data might be too long or invalid.")?;

    // Print to terminal if requested
    if args.print {
        println!("\n{}", render_qr_to_terminal(&code));
    }

    // Save to file if output path is provided
    if let Some(output_path) = args.output {
        // Render to image
        let image = code.render::<Luma<u8>>()
            .min_dimensions(args.scale, args.scale)
            .quiet_zone(true)
            .module_dimensions(args.scale, args.scale)
            .build();

        // Save to file
        image.save(&output_path)
            .context(format!("Failed to save QR code to {:?}", output_path))?;

        println!("✓ QR code successfully saved to: {:?}", output_path);
        println!("  Size: {}x{} pixels", image.width(), image.height());
    }
    
    Ok(())
}

/// Renders a QR code to a string suitable for terminal display
/// Uses Unicode block characters to create a visual representation
fn render_qr_to_terminal(code: &QrCode) -> String {
    code.render::<char>()
        .quiet_zone(false)
        .module_dimensions(2, 1)
        .dark_color('█')
        .light_color(' ')
        .build()
}

#[cfg(test)]
mod tests {
    use qrcode::QrCode;

    #[test]
    fn test_qrcode_generation() {
        let result = QrCode::new("Hello, World!");
        assert!(result.is_ok());
    }

    #[test]
    fn test_qrcode_with_url() {
        let result = QrCode::new("https://www.rust-lang.org");
        assert!(result.is_ok());
    }

    #[test]
    fn test_qrcode_with_email() {
        let result = QrCode::new("mailto:test@example.com");
        assert!(result.is_ok());
    }
}

