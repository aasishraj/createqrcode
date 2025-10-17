# createqrcode

A CLI tool to convert text data into QR codes.

## Installation

```bash
cargo install createqrcode
```

## Usage

```bash
# Generate QR code and save to file
createqrcode --data "Hello World" --output qr.png

# Print QR code to terminal
createqrcode --data "https://example.com" --print

# Custom scale and border
createqrcode --data "My data" --output qr.png --scale 20 --border 8
```

## Options

- `-d, --data <DATA>` - Text to encode
- `-o, --output <OUTPUT>` - Output file path (PNG)
- `-s, --scale <SCALE>` - Scale factor (default: 10)
- `-b, --border <BORDER>` - Border size in modules (default: 4)
- `-p, --print` - Print QR code to terminal

## License

MIT
